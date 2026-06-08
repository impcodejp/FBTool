import type { UseFormReturn } from "react-hook-form";
import type { HeaderSchema } from "../hooks/useHeaderForm";

interface Props {
  form: UseFormReturn<HeaderSchema>;
  onExport: () => void;
  onImport: () => void;
}

function Field({
  label,
  error,
  children,
}: {
  label: string;
  error?: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <label className="block text-xs text-gray-500 mb-0.5">{label}</label>
      {children}
      {error && <p className="mt-0.5 text-xs text-red-500">{error}</p>}
    </div>
  );
}

const inputCls =
  "w-full border border-gray-300 rounded px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-orange-400 focus:border-orange-400 bg-white";
const inputErrCls =
  "w-full border border-red-400 rounded px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-red-400 bg-white";

export function HeaderForm({ form, onExport, onImport }: Props) {
  const {
    register,
    formState: { errors },
  } = form;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* パネルヘッダー */}
      <div className="flex items-center justify-between px-3 py-2 bg-orange-50 border-b border-orange-200 flex-shrink-0">
        <span className="text-xs font-semibold text-gray-600">ヘッダー情報</span>
        <div className="flex items-center gap-1.5">
          <button
            type="button"
            onClick={onImport}
            className="text-xs text-blue-600 hover:text-blue-800 hover:underline px-1"
          >
            取込
          </button>
          <span className="text-gray-300 text-xs">|</span>
          <button
            type="button"
            onClick={onExport}
            className="text-xs text-blue-600 hover:text-blue-800 hover:underline px-1"
          >
            出力
          </button>
        </div>
      </div>

      {/* フォームフィールド */}
      <div className="flex-1 overflow-y-auto p-3 space-y-3">
        <Field label="入金日（YYYY/MM/DD）" error={errors.payment_date?.message}>
          <input
            {...register("payment_date")}
            placeholder="2026/06/06"
            className={errors.payment_date ? inputErrCls : inputCls}
          />
        </Field>

        <div className="grid grid-cols-2 gap-2">
          <Field label="銀行コード" error={errors.bank_code?.message}>
            <input
              {...register("bank_code")}
              placeholder="0001"
              maxLength={4}
              className={errors.bank_code ? inputErrCls : inputCls}
            />
          </Field>
          <Field label="銀行名（半角）" error={errors.bank_name?.message}>
            <input
              {...register("bank_name")}
              placeholder="ﾐｽﾞﾎ"
              maxLength={15}
              className={errors.bank_name ? inputErrCls : inputCls}
            />
          </Field>
        </div>

        <div className="grid grid-cols-2 gap-2">
          <Field label="支店コード" error={errors.branch_code?.message}>
            <input
              {...register("branch_code")}
              placeholder="001"
              maxLength={3}
              className={errors.branch_code ? inputErrCls : inputCls}
            />
          </Field>
          <Field label="支店名（半角）" error={errors.branch_name?.message}>
            <input
              {...register("branch_name")}
              placeholder="ﾄｳｷｮｳ"
              maxLength={15}
              className={errors.branch_name ? inputErrCls : inputCls}
            />
          </Field>
        </div>

        <Field label="預金種目" error={errors.deposit_type?.message}>
          <select {...register("deposit_type")} className={inputCls}>
            <option value="1">普通預金</option>
            <option value="2">当座預金</option>
          </select>
        </Field>

        <Field label="口座番号（7桁以内）" error={errors.account_number?.message}>
          <input
            {...register("account_number")}
            placeholder="1234567"
            maxLength={7}
            className={errors.account_number ? inputErrCls : inputCls}
          />
        </Field>
      </div>
    </div>
  );
}
