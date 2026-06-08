import type { CsvValidationError } from "../types/csvRecord";

interface Props {
  errors: CsvValidationError[];
}

export function ValidationErrors({ errors }: Props) {
  if (errors.length === 0) return null;

  return (
    <div className="bg-red-50 border border-red-200 rounded shadow-sm">
      <div className="px-3 py-2 border-b border-red-200 bg-red-100">
        <span className="text-xs font-semibold text-red-700">
          ⚠ CSVバリデーションエラー（{errors.length} 件）
        </span>
      </div>
      <ul className="px-3 py-2 space-y-0.5 max-h-28 overflow-y-auto">
        {errors.map((e, i) => (
          <li key={i} className="text-xs text-red-600">
            <span className="font-medium">行{e.row}</span>{" "}
            <span className="text-red-400">[{e.field}]</span> {e.message}
          </li>
        ))}
      </ul>
    </div>
  );
}
