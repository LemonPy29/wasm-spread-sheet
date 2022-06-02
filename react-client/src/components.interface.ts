import { Option } from "fp-ts/lib/Option";
import { MouseEvent } from "react";

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

export interface SideBarProps {
  schema: Schema;
  onClick: (ev: MouseEvent) => void;
}

export interface SchemaUIProps {
  schema: Schema;
}

export interface ColumnProps {
  header: string;
  data?: string[];
}

export interface FrameProps {
  header?: string[];
  data?: string[][];
}

type Schema = Option<Record<string, string>>;
