export interface ArchiveEntry {
  path: string;
  name: string;
  is_directory: boolean;
  compressed_size: number;
  uncompressed_size: number;
  modified: string | null;
  compression_method: string;
  crc32: number | null;
  encrypted: boolean;
}

export interface OpenArchiveResponse {
  entries: ArchiveEntry[];
  format: string;
  path: string;
  supports_modification: boolean;
}

export interface ProgressPayload {
  current: number;
  total: number;
  message: string;
}

export type SortColumn = "name" | "size" | "packed" | "ratio" | "type" | "modified";
export type SortOrder = "asc" | "desc";
