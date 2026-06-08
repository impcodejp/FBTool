import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";

const HALFWIDTH_RE = /^[\x20-\x7E｡-ﾟ]+$/;

function isValidDate(s: string): boolean {
  const [y, m, d] = s.split("/").map(Number);
  if (!y || !m || !d) return false;
  const dt = new Date(y, m - 1, d);
  return dt.getFullYear() === y && dt.getMonth() === m - 1 && dt.getDate() === d;
}

const schema = z.object({
  payment_date: z
    .string()
    .regex(/^\d{4}\/\d{2}\/\d{2}$/, "日付は YYYY/MM/DD 形式で入力してください")
    .refine(isValidDate, "有効な日付を入力してください"),
  bank_code: z.string().regex(/^\d{4}$/, "4 桁の数字を入力してください"),
  bank_name: z
    .string()
    .min(1, "銀行名を入力してください")
    .refine((s) => HALFWIDTH_RE.test(s), "半角カタカナ・半角英数字のみ使用できます")
    .refine((s) => s.length <= 15, "15 文字以内で入力してください"),
  branch_code: z.string().regex(/^\d{3}$/, "3 桁の数字を入力してください"),
  branch_name: z
    .string()
    .min(1, "支店名を入力してください")
    .refine((s) => HALFWIDTH_RE.test(s), "半角カタカナ・半角英数字のみ使用できます")
    .refine((s) => s.length <= 15, "15 文字以内で入力してください"),
  deposit_type: z.enum(["1", "2"]),
  account_number: z.string().regex(/^\d{1,7}$/, "7 桁以内の数字を入力してください"),
});

export type HeaderSchema = z.infer<typeof schema>;

export function useHeaderForm() {
  return useForm<HeaderSchema>({
    resolver: zodResolver(schema),
    defaultValues: {
      payment_date: "",
      bank_code: "",
      bank_name: "",
      branch_code: "",
      branch_name: "",
      deposit_type: "1",
      account_number: "",
    },
    mode: "onChange",
  });
}
