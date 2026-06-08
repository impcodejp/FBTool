export interface HeaderFormData {
  payment_date: string;
  bank_code: string;
  bank_name: string;
  branch_code: string;
  branch_name: string;
  deposit_type: string;
  account_number: string;
}

/** Tauri コマンドに渡す型（deposit_type を数値に変換済み） */
export interface HeaderInfo {
  payment_date: string;
  bank_code: string;
  bank_name: string;
  branch_code: string;
  branch_name: string;
  deposit_type: number;
  account_number: string;
}
