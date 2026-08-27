import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { message, save } from "@tauri-apps/plugin-dialog";
import type { UseFormReturn } from "react-hook-form";

import type { HeaderSchema } from "../hooks/useHeaderForm";
import { useDepositWithdrawalCsvUpload } from "../hooks/useDepositWithdrawalCsvUpload";
import { CsvUploader } from "./CsvUploader";
import { DepositWithdrawalPreviewTable } from "./DepositWithdrawalPreviewTable";
import { ValidationErrors } from "./ValidationErrors";
import { GenerateButton } from "./GenerateButton";

interface Props {
  headerForm: UseFormReturn<HeaderSchema>;
}

export function DepositWithdrawalPanel({ headerForm }: Props) {
  const csvUpload = useDepositWithdrawalCsvUpload();

  const [outputPath, setOutputPath] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [generateError, setGenerateError] = useState<string | null>(null);

  const csvErrors = csvUpload.readResult?.errors ?? [];
  const canGenerate =
    headerForm.formState.isValid &&
    csvUpload.csvPath !== null &&
    outputPath !== null &&
    csvErrors.length === 0 &&
    (csvUpload.readResult?.total_count ?? 0) > 0 &&
    !isGenerating;

  const handleExportTemplate = async () => {
    const path = await save({
      title: "CSVテンプレートの保存先を選択",
      filters: [{ name: "CSV", extensions: ["csv"] }],
      defaultPath: "template_deposit_withdrawal.csv",
    });
    if (!path) return;

    try {
      await invoke("export_deposit_withdrawal_csv_template", { outputPath: path });
      await message("CSVテンプレートを出力しました", { title: "完了", kind: "info" });
    } catch (err) {
      await message(String(err), { title: "エラー", kind: "error" });
    }
  };

  const handleSelectOutput = async () => {
    const path = await save({
      title: "保存先を選択",
      filters: [{ name: "テキストファイル", extensions: ["txt"] }],
      defaultPath: "FB_nyushukkin.txt",
    });
    if (path) setOutputPath(path);
  };

  const handleGenerate = headerForm.handleSubmit(async (data) => {
    if (!csvUpload.csvPath || !outputPath) return;

    setIsGenerating(true);
    setGenerateError(null);

    try {
      await invoke("generate_fb_deposit_withdrawal", {
        headerInfo: {
          payment_date: data.payment_date,
          bank_code: data.bank_code,
          bank_name: data.bank_name,
          branch_code: data.branch_code,
          branch_name: data.branch_name,
          deposit_type: parseInt(data.deposit_type, 10),
          account_number: data.account_number,
        },
        csvPath: csvUpload.csvPath,
        outputPath,
      });

      await message("FBデータの生成が完了しました", { title: "完了", kind: "info" });
    } catch (err) {
      setGenerateError(String(err));
    } finally {
      setIsGenerating(false);
    }
  });

  return (
    <div className="flex-1 flex flex-col gap-2.5 overflow-hidden min-w-0">
      <CsvUploader
        csvPath={csvUpload.csvPath}
        totalCount={csvUpload.readResult?.total_count ?? null}
        isLoading={csvUpload.isLoading}
        loadError={csvUpload.loadError}
        onSelect={csvUpload.selectAndReadCsv}
        onExportTemplate={handleExportTemplate}
        title="入出金明細 CSV"
        columnsHint="文字コード: Shift-JIS　列: 入払区分, 取引区分, 取引金額, 摘要文字列, 仕向銀行名(カナ), 仕向店名(カナ), 摘要内容, EDI"
      />

      <DepositWithdrawalPreviewTable
        rows={csvUpload.readResult?.preview ?? []}
        totalCount={csvUpload.readResult?.total_count ?? 0}
      />

      {csvErrors.length > 0 && <ValidationErrors errors={csvErrors} />}

      <GenerateButton
        outputPath={outputPath}
        onSelectOutput={handleSelectOutput}
        onGenerate={handleGenerate}
        canGenerate={canGenerate}
        isGenerating={isGenerating}
        generateError={generateError}
      />
    </div>
  );
}
