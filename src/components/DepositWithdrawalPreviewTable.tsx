import type { DepositWithdrawalPreviewRow } from "../types/depositWithdrawalRecord";

interface Props {
  rows: DepositWithdrawalPreviewRow[];
  totalCount: number;
}

const HEADERS = [
  "行",
  "入払区分",
  "取引区分",
  "取引金額",
  "摘要文字列",
  "仕向銀行名",
  "仕向店名",
  "摘要内容",
  "EDI",
];

const TRANSACTION_FLAG_LABEL: Record<string, string> = {
  "1": "入金",
  "2": "出金",
};

export function DepositWithdrawalPreviewTable({ rows, totalCount }: Props) {
  return (
    <div className="bg-white border border-gray-200 rounded shadow-sm flex flex-col min-h-0 flex-1">
      <div className="px-3 py-2 bg-orange-50 border-b border-orange-200 flex-shrink-0">
        <span className="text-xs font-semibold text-gray-600">
          {rows.length > 0 ? `プレビュー（全 ${totalCount} 件）` : "プレビュー"}
        </span>
      </div>

      <div className="overflow-auto flex-1 min-h-0">
        <table className="w-full text-xs border-collapse">
          <thead className="sticky top-0 z-10">
            <tr className="bg-orange-50 border-b border-orange-200">
              {HEADERS.map((h) => (
                <th
                  key={h}
                  className="px-2 py-1.5 text-left font-medium text-gray-500 whitespace-nowrap border-r border-gray-200 last:border-r-0"
                >
                  {h}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.length === 0 ? (
              <tr>
                <td
                  colSpan={HEADERS.length}
                  className="px-2 py-6 text-center text-xs text-gray-400"
                >
                  CSV ファイルを選択するとここにデータが表示されます
                </td>
              </tr>
            ) : (
              rows.map((r) => (
                <tr
                  key={r.row}
                  className="h-7 hover:bg-orange-50 border-b border-gray-100 last:border-0"
                >
                  <td className="px-2 text-gray-400 border-r border-gray-200">{r.row}</td>
                  <td className="px-2 border-r border-gray-200">
                    {TRANSACTION_FLAG_LABEL[r.transaction_flag] ?? r.transaction_flag}
                  </td>
                  <td className="px-2 border-r border-gray-200">{r.transaction_category}</td>
                  <td className="px-2 text-right tabular-nums border-r border-gray-200">
                    {Number(r.amount).toLocaleString()}
                  </td>
                  <td className="px-2 max-w-48 truncate border-r border-gray-200">
                    {r.description}
                  </td>
                  <td className="px-2 border-r border-gray-200">{r.bank_name}</td>
                  <td className="px-2 border-r border-gray-200">{r.branch_name}</td>
                  <td className="px-2 border-r border-gray-200">{r.summary_content}</td>
                  <td className="px-2">{r.edi}</td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
