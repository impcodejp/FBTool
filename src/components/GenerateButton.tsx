interface Props {
  outputPath: string | null;
  onSelectOutput: () => void;
  onGenerate: () => void;
  canGenerate: boolean;
  isGenerating: boolean;
  generateError: string | null;
}

export function GenerateButton({
  outputPath,
  onSelectOutput,
  onGenerate,
  canGenerate,
  isGenerating,
  generateError,
}: Props) {
  return (
    <div className="bg-white border border-gray-200 rounded shadow-sm flex-shrink-0">
      <div className="px-3 py-2 bg-orange-50 border-b border-orange-200">
        <span className="text-xs font-semibold text-gray-600">出力先</span>
      </div>

      <div className="px-3 py-2 space-y-2">
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={onSelectOutput}
            className="shrink-0 bg-white hover:bg-gray-50 border border-gray-300 rounded px-3 py-1 text-xs font-medium text-gray-700 transition-colors"
          >
            保存先を選択
          </button>
          <span className="text-xs text-gray-500 truncate">
            {outputPath ?? "出力ファイルのパスを選択してください"}
          </span>
        </div>

        {generateError && (
          <div className="bg-red-50 border border-red-200 rounded px-2 py-1.5">
            <p className="text-xs font-medium text-red-700 mb-0.5">生成エラー</p>
            <pre className="text-xs text-red-600 whitespace-pre-wrap">{generateError}</pre>
          </div>
        )}

        <div className="flex items-center justify-between pt-1">
          {!canGenerate && !isGenerating && (
            <span className="text-xs text-gray-400">
              必須項目・CSV・出力先をすべて設定してください
            </span>
          )}
          <div className="ml-auto">
            <button
              type="button"
              onClick={onGenerate}
              disabled={!canGenerate}
              className="bg-orange-500 hover:bg-orange-600 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-semibold px-6 py-1.5 rounded text-sm transition-colors"
            >
              {isGenerating ? "生成中..." : "FBデータを生成"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
