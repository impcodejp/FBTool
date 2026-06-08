export interface CsvPreviewRow {
  row: number;
  amount: string;
  bank_name: string;
  branch_name: string;
  description: string;
  edi: string;
}

export interface CsvValidationError {
  row: number;
  field: string;
  message: string;
}

export interface CsvReadResult {
  preview: CsvPreviewRow[];
  total_count: number;
  errors: CsvValidationError[];
}
