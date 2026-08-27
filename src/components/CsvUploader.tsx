interface Props {
  csvPath: string | null;
  totalCount: number | null;
  isLoading: boolean;
  loadError: string | null;
  onSelect: () => void;
  onExportTemplate: () => void;
  title?: string;
  columnsHint?: string;
}

export function CsvUploader({
  csvPath,
  totalCount,
  isLoading,
  loadError,
  onSelect,
  onExportTemplate,
  title = "入金明細 CSV",
  columnsHint = "文字コード: Shift-JIS　列: 金額, 銀行名(カナ), 支店名(カナ), 摘要文字列, EDI",
}: Props) {
  return (
    <div className="bg-white border border-gray-200 rounded shadow-sm">
      <div className="flex items-center justify-between px-3 py-2 bg-orange-50 border-b border-orange-200">
        <span className="text-xs font-semibold text-gray-600">{title}</span>
        <button
          type="button"
          onClick={onExportTemplate}
          className="text-xs text-blue-600 hover:text-blue-800 hover:underline"
        >
          CSVテンプレートを出力
        </button>
      </div>

      <div className="px-3 py-2 space-y-2">
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={onSelect}
            disabled={isLoading}
            className="shrink-0 bg-white hover:bg-gray-50 border border-gray-300 rounded px-3 py-1 text-xs font-medium text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? "読み込み中..." : "ファイルを選択"}
          </button>
          <span className="text-xs text-gray-500 truncate">
            {csvPath ?? "CSV ファイルを選択してください"}
          </span>
          {totalCount !== null && (
            <span className="shrink-0 text-xs text-gray-400 ml-auto">{totalCount} 件</span>
          )}
        </div>

        {loadError && (
          <p className="text-xs text-red-600 bg-red-50 border border-red-200 rounded px-2 py-1">
            {loadError}
          </p>
        )}

        <p className="text-xs text-gray-400">{columnsHint}</p>
      </div>
    </div>
  );
}
