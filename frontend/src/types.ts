/* This file is generated and managed by tsync */

export interface UsageData {
  used: number;
  total: number;
  percent: number;
}

export interface SystemData {
  cpu: number;
  ram: UsageData;
  swap: UsageData;
}
