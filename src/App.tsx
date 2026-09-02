import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { resolveResource } from "@tauri-apps/api/path";
import { message, open, save } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";

import { useHeaderForm } from "./hooks/useHeaderForm";
import type { HeaderSchema } from "./hooks/useHeaderForm";
import { HeaderForm } from "./components/HeaderForm";
import { FbKindTabs } from "./components/FbKindTabs";
import { FurikomiPanel } from "./components/FurikomiPanel";
import { DepositWithdrawalPanel } from "./components/DepositWithdrawalPanel";
import type { FbKind } from "./types/fbKind";

function App() {
  const headerForm = useHeaderForm();
  const [fbKind, setFbKind] = useState<FbKind>("furikomi");

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

  // ---- システムマニュアルを開く ----
  const handleOpenManual = async () => {
    try {
      const manualPath = await resolveResource("docs/ユーザーマニュアル.html");
      await openPath(manualPath);
    } catch (err) {
      await message(String(err), { title: "エラー", kind: "error" });
    }
  };

  return (
    <div className="h-screen flex flex-col bg-gray-200 overflow-hidden">
      {/* アプリヘッダー */}
      <header className="bg-orange-500 text-white px-4 py-2 flex items-center gap-3 flex-shrink-0 shadow-md">
        <h1 className="font-bold text-sm tracking-wide">FB入金データ生成ツール</h1>
        <span className="text-orange-100 text-xs">全銀協フォーマット準拠</span>
        <button
          type="button"
          onClick={handleOpenManual}
          className="ml-auto text-xs bg-orange-600 hover:bg-orange-700 px-3 py-1 rounded"
        >
          マニュアルを開く
        </button>
      </header>

      {/* FB種別タブ */}
      <FbKindTabs value={fbKind} onChange={setFbKind} />

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

        {/* 右パネル: CSV・プレビュー・生成（FB種別ごとに出し分け） */}
        {fbKind === "furikomi" ? (
          <FurikomiPanel key="furikomi" headerForm={headerForm} />
        ) : (
          <DepositWithdrawalPanel key="deposit_withdrawal" headerForm={headerForm} />
        )}
      </div>
    </div>
  );
}

export default App;
