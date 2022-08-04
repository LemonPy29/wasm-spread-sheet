import { ComponentPropsWithoutRef } from "react";

export type DataStatus = "Empty" | "Waiting" | "headerPhase" | "Usable";

export interface DataStatusManager {
  dataStatus: DataStatus;
  setDataStatus: (input: DataStatus) => void;
}

export interface Metadata {
  headerChecked: boolean;
  headerCheckBoxDisabled: boolean;
}

export interface MetadataManager {
  metadata: Metadata;
  setMetadata: (input: Metadata) => void;
}

export interface CheckBoxProps {
  checked: boolean;
  disabled: boolean;
  onChange?: React.ChangeEventHandler<HTMLInputElement>;
}

export interface HeaderProps {
  type: "left" | "right" | "center";
  name: string;
}

export interface ColumnProps extends ComponentPropsWithoutRef<"div"> {
  header: HeaderProps;
  data?: string[];
}

export interface FrameProps {
  header?: string[];
  data?: string[][];
}
