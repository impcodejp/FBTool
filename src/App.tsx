import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { message, open, save } from "@tauri-apps/plugin-dialog";

import { useHeaderForm } from "./hooks/useHeaderForm";
import type { HeaderSchema } from "./hooks/useHeaderForm";
import { useCsvUpload } from "./hooks/useCsvUpload";
import { HeaderForm } from "./components/HeaderForm";
import { CsvUploader } from "./components/CsvUploader";
import { PreviewTable } from "./components/PreviewTable";
import { ValidationErrors } from "./components/ValidationErrors";
import { GenerateButton } from "./components/GenerateButton";

function App() {
  const headerForm = useHeaderForm();
  const csvUpload = useCsvUpload();

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

  // ---- ヘッダ情報 出力 ----
  const handleExportHeader = async () => {
    const path = await save({
      title: "ヘッダ情報の保存先を選択",
      filters: [{ name: "ヘッダ情報", extensions: ["json"] }],
      defaultPath: "header_info.json",
    });
    if (!path) return;

    const data = headerForm.getValues();
    try {
      await invoke("export_header_info", { data, outputPath: path });
      await message("ヘッダ情報を出力しました", { title: "完了", kind: "info" });
    } catch (err) {
      await message(String(err), { title: "エラー", kind: "error" });
    }
  };

  // ---- ヘッダ情報 取込 ----
  const handleImportHeader = async () => {
    const selected = await open({
      title: "ヘッダ情報ファイルを選択",
      filters: [{ name: "ヘッダ情報", extensions: ["json"] }],
      multiple: false,
    });
    if (!selected) return;

    const filePath = Array.isArray(selected) ? selected[0] : selected;
    try {
      const data = await invoke<HeaderSchema>("import_header_info", { filePath });
      headerForm.reset(data);
      await headerForm.trigger();
    } catch (err) {
      await message(String(err), { title: "エラー", kind: "error" });
    }
  };

  // ---- CSV テンプレート出力 ----
  const handleExportTemplate = async () => {
    const path = await save({
      title: "CSVテンプレートの保存先を選択",
      filters: [{ name: "CSV", extensions: ["csv"] }],
      defaultPath: "template_FB.csv",
    });
    if (!path) return;

    try {
      await invoke("export_csv_template", { outputPath: path });
      await message("CSVテンプレートを出力しました", { title: "完了", kind: "info" });
    } catch (err) {
      await message(String(err), { title: "エラー", kind: "error" });
    }
  };

  // ---- 出力先選択 ----
  const handleSelectOutput = async () => {
    const path = await save({
      title: "保存先を選択",
      filters: [{ name: "テキストファイル", extensions: ["txt"] }],
      defaultPath: "FB.txt",
    });
    if (path) setOutputPath(path);
  };

  // ---- FB 生成 ----
  const handleGenerate = headerForm.handleSubmit(async (data) => {
    if (!csvUpload.csvPath || !outputPath) return;

    setIsGenerating(true);
    setGenerateError(null);

    try {
      await invoke("generate_fb", {
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
    <div className="h-screen flex flex-col bg-gray-200 overflow-hidden">
      {/* アプリヘッダー */}
      <header className="bg-orange-500 text-white px-4 py-2 flex items-center gap-3 flex-shrink-0 shadow-md">
        <h1 className="font-bold text-sm tracking-wide">FB入金データ生成ツール</h1>
        <span className="text-orange-100 text-xs">全銀協フォーマット準拠</span>
      </header>

      {/* メインエリア（2カラム） */}
      <div className="flex flex-1 overflow-hidden p-2.5 gap-2.5">
        {/* 左パネル: ヘッダー情報 */}
        <aside className="w-64 bg-white border border-gray-300 rounded shadow-sm flex flex-col flex-shrink-0 overflow-hidden">
          <HeaderForm
            form={headerForm}
            onExport={handleExportHeader}
            onImport={handleImportHeader}
          />
        </aside>

        {/* 右パネル: CSV・プレビュー・生成 */}
        <div className="flex-1 flex flex-col gap-2.5 overflow-hidden min-w-0">
          {/* 上部固定: CSV選択 */}
          <CsvUploader
            csvPath={csvUpload.csvPath}
            totalCount={csvUpload.readResult?.total_count ?? null}
            isLoading={csvUpload.isLoading}
            loadError={csvUpload.loadError}
            onSelect={csvUpload.selectAndReadCsv}
            onExportTemplate={handleExportTemplate}
          />

          {/* 中部伸縮: プレビュー（残りスペースをすべて使う） */}
          <PreviewTable
            rows={csvUpload.readResult?.preview ?? []}
            totalCount={csvUpload.readResult?.total_count ?? 0}
          />

          {/* バリデーションエラー（あるときのみ表示） */}
          {csvErrors.length > 0 && <ValidationErrors errors={csvErrors} />}

          {/* 下部固定: 出力先 + 生成ボタン */}
          <GenerateButton
            outputPath={outputPath}
            onSelectOutput={handleSelectOutput}
            onGenerate={handleGenerate}
            canGenerate={canGenerate}
            isGenerating={isGenerating}
            generateError={generateError}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
