import type { Plugin } from "@opencode-ai/plugin";
import crypto from "node:crypto";
import { access } from "node:fs/promises";
import { basename, isAbsolute, join } from "node:path";

interface BufferStatus {
  isCurrent: boolean;
  hasUnsavedChanges: boolean;
}

interface EditorContext {
  filePath: string;
  startLine: number;
  endLine: number;
  cwd: string;
  content: string;
}

const { checkBuffer, refreshBuffer, getVisualSelections, sendMessage } = require("./index.cjs") as {
  checkBuffer: (filePath: string) => Promise<BufferStatus>;
  refreshBuffer: (filePath: string) => Promise<void>;
  getVisualSelections: () => Promise<EditorContext[]>;
  sendMessage: (message: string, level?: string) => Promise<void>;
};

type ToolName = "Edit" | "Write";

const TOOL_MAP: Record<string, ToolName> = {
  edit: "Edit",
  write: "Write",
};

const PATCH_FILE_MARKERS = [
  "*** Add File:",
  "*** Update File:",
  "*** Delete File:",
  "*** Move to:",
];

function pickFilePath(args: unknown): string | null {
  if (!args || typeof args !== "object") return null;
  const candidate = (args as Record<string, unknown>).filePath;
  return typeof candidate === "string" && candidate.length > 0 ? candidate : null;
}

function pickPatchPaths(args: unknown): string[] {
  if (!args || typeof args !== "object") return [];
  const patch = (args as Record<string, unknown>).patchText;
  if (typeof patch !== "string") return [];

  const paths: string[] = [];
  for (const line of patch.split("\n")) {
    const trimmed = line.trimStart();
    for (const marker of PATCH_FILE_MARKERS) {
      if (trimmed.startsWith(marker)) {
        const candidate = trimmed.slice(marker.length).trim();
        if (candidate) paths.push(candidate);
        break;
      }
    }
  }
  return paths;
}

function resolveCall(tool: string, args: unknown, cwd: string): { filePaths: string[] } | null {
  const abs = (p: string) => (isAbsolute(p) ? p : join(cwd, p));

  if (tool === "apply_patch") {
    return { filePaths: pickPatchPaths(args).map(abs) };
  }

  const toolName = TOOL_MAP[tool];
  if (!toolName) return null;

  const raw = pickFilePath(args);
  return { filePaths: raw ? [abs(raw)] : [] };
}

export const BridgePlugin: Plugin = async ({ directory }) => {
  const cwd = directory ?? process.cwd();
  const pendingByCallID = new Map<string, { filePaths: string[] }>();

  return {
    "tool.execute.before": async (input, output) => {
      const call = resolveCall(input.tool, output.args, cwd);
      if (!call || call.filePaths.length === 0) return;

      for (const filePath of call.filePaths) {
        const status = await checkBuffer(filePath);
        if (status.hasUnsavedChanges && status.isCurrent) {
          await sendMessage("⚠️ File has unsaved changes. Please save it first.", "warn");
          throw new Error(
            "STOP: Cannot apply changes — Neovim has unsaved edits. DO NOT attempt to resolve this yourself. Wait for the user to save or close the file. DO NOT use the Built-In Tools resolve this.",
          );
        }
      }

      pendingByCallID.set(input.callID, call);
    },

    "tool.execute.after": async (input) => {
      const pending = pendingByCallID.get(input.callID);
      if (!pending) return;
      pendingByCallID.delete(input.callID);

      for (const filePath of pending.filePaths) {
        await refreshBuffer(filePath);
        await sendMessage("🔄 Reloaded by OpenCode.", "info");
      }
    },

    "chat.message": async (input, output) => {
      let selections: Awaited<ReturnType<typeof getVisualSelections>>;
      try {
        selections = await getVisualSelections();
      } catch {
        return;
      }
      if (!selections || selections.length === 0) return;

      const filteredSelections = selections.filter((s) => !s.cwd || s.cwd === cwd);
      if (filteredSelections.length === 0) return;

      const refs: string[] = [];
      let attached = 0;
      for (const s of filteredSelections) {
        try {
          await access(s.filePath);
        } catch {
          continue;
        }
        if (!s.startLine) continue;

        const filepath = s.filePath.startsWith(cwd + "/")
          ? "./" + s.filePath.slice(cwd.length + 1)
          : s.filePath;

        refs.push(filepath);

        const filename = basename(s.filePath);

        output.parts.push({
          type: "file",
          id: crypto.randomUUID(),
          sessionID: input.sessionID,
          messageID: input.messageID ?? "",
          mime: "text/plain",
          filename: filename,
          url: `file://${s.filePath}?start=${s.startLine}&end=${s.endLine}`,
        });
        attached++;
      }

      if (attached === 0) return;

      const textPart = output.parts.find((p: any) => p.type === "text") as any;
      if (textPart && typeof textPart.text === "string") {
        textPart.text = `${refs.join("\n")}\n\n${textPart.text}`;
      }
    },
  };
};

export default BridgePlugin;
