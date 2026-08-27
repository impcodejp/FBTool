import { FB_KIND_LABEL, type FbKind } from "../types/fbKind";

interface Props {
  value: FbKind;
  onChange: (kind: FbKind) => void;
}

const KINDS: FbKind[] = ["furikomi", "deposit_withdrawal"];

export function FbKindTabs({ value, onChange }: Props) {
  return (
    <div className="flex gap-1 px-2.5 pt-2 flex-shrink-0">
      {KINDS.map((kind) => {
        const active = kind === value;
        return (
          <button
            key={kind}
            type="button"
            onClick={() => onChange(kind)}
            className={
              active
                ? "px-4 py-1.5 text-sm font-semibold rounded-t bg-white text-orange-600 border border-b-0 border-gray-300"
                : "px-4 py-1.5 text-sm font-medium rounded-t bg-orange-100 text-gray-500 hover:bg-orange-200 border border-b-0 border-transparent"
            }
          >
            {FB_KIND_LABEL[kind]}
          </button>
        );
      })}
    </div>
  );
}
