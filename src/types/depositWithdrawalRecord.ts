import type { CsvValidationError } from "./csvRecord";

export interface DepositWithdrawalPreviewRow {
  row: number;
  transaction_flag: string;
  transaction_category: string;
  amount: string;
  description: string;
  bank_name: string;
  branch_name: string;
  summary_content: string;
  edi: string;
}

export interface DepositWithdrawalReadResult {
  preview: DepositWithdrawalPreviewRow[];
  total_count: number;
  errors: CsvValidationError[];
}
