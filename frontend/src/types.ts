/* This file is generated and managed by tsync */

export type BinarySuffix =
  | "B" | "KiB" | "MiB" | "GiB" | "TiB" | "PiB";

export interface PrettyBytesBinary {
  num: number;
  suffix: BinarySuffix;
}

export interface UsageData {
  used: PrettyBytesBinary;
  total: PrettyBytesBinary;
  percent: number;
}

export interface System {
  cpu: number;
  ram: UsageData;
  swap: UsageData;
}
