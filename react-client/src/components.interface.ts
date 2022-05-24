import { Option } from "fp-ts/lib/Option";

export type DataStatus = "Empty" | "Waiting" | "Usable";

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
}

export interface SchemaUIProps {
  schema: Schema;
}

export interface ColumnProps { header: string; data?: string[] };
export interface FrameProps { header?: string[]; data?: string[][] };

type Schema = Option<Record<string, string>>;
