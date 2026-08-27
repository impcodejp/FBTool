export type FbKind = "furikomi" | "deposit_withdrawal";

export const FB_KIND_LABEL: Record<FbKind, string> = {
  furikomi: "振込入金明細",
  deposit_withdrawal: "入出金明細",
};
