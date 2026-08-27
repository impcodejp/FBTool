import { useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { DepositWithdrawalReadResult } from "../types/depositWithdrawalRecord";

export function useDepositWithdrawalCsvUpload() {
  const [csvPath, setCsvPath] = useState<string | null>(null);
  const [readResult, setReadResult] = useState<DepositWithdrawalReadResult | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [loadError, setLoadError] = useState<string | null>(null);

  const selectAndReadCsv = useCallback(async () => {
    const selected = await open({
      title: "CSVファイルを選択",
      filters: [{ name: "CSV", extensions: ["csv"] }],
      multiple: false,
    });

    if (!selected) return;
    const path = Array.isArray(selected) ? selected[0] : selected;

    setCsvPath(path);
    setIsLoading(true);
    setLoadError(null);
    setReadResult(null);

    try {
      const result = await invoke<DepositWithdrawalReadResult>(
        "read_deposit_withdrawal_csv_records",
        { csvPath: path },
      );
      setReadResult(result);
    } catch (err) {
      setLoadError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, []);

  return { csvPath, readResult, isLoading, loadError, selectAndReadCsv };
}
