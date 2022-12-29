import { ComponentPropsWithoutRef, Dispatch, SetStateAction } from "react";

export interface MenuItemProps extends ComponentPropsWithoutRef<"li"> {
  withSubMenu?: boolean;
  label: string;
}

export type ContextMenuStatus = "Open" | "Closed";

export interface ContextMenuProps {
  xPos: number;
  yPos: number;
  columnClickedName: string;
  status: ContextMenuStatus;
  setStatus: Dispatch<SetStateAction<ContextMenuStatus>>;
}

export interface SliderProps {
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
