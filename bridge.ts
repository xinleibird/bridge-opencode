import type { Plugin } from "@opencode-ai/plugin";
import { isAbsolute, join } from "node:path";
import { checkBuffer, refreshBuffer, getVisualSelections } from "./index.cjs";

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
      }
    },

    "chat.message": async (_input, output) => {
      let selections;
      try {
        selections = await getVisualSelections();
      } catch {
        return;
      }
      if (!selections || selections.length === 0) return;

      const filteredSelections = selections.filter((s) => !s.cwd || s.cwd === cwd);
      if (filteredSelections.length === 0) return;

      const textPart = output.parts.find((p: any) => p.type === "text") as any;
      if (!textPart || typeof textPart.text !== "string") return;

      const lines = filteredSelections.map((s) => {
        const path = s.filePath.startsWith(cwd + "/")
          ? "./" + s.filePath.slice(cwd.length + 1)
          : s.filePath;
        return `${path}:${s.startLine}-${s.endLine}`;
      });

      textPart.text = `${lines.join("\n")}\n\n${textPart.text}`;
    },
  };
};

export default BridgePlugin;
