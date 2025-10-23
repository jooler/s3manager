import type { Selected } from "bits-ui";
import type { Snippet } from "svelte";

export interface Bucket {
  id?: number;
  type: "r2" | "s3" | "oss";
  bucketName: string;
  accountId: string;
  accessKey: string;
  secretKey: string;
  customDomain: string;
  s3Api?: string;
  endpoint?: string;
  region?: string;
  [key: string]: string | number | undefined;
}

export interface File {
  type: "text" | "image" | "file";
  id: string;
  source: { filePath: string } | { fileContent: string };
  remoteFilename: string;
  remoteFilenamePrefix: string;
}

export interface FileDetail {
  id: string;
  path: string;
  relativePath: string;
  isDir: boolean;
}

export type UploadStatus =
  | "success"
  | "cancelled"
  | {
      uploading: {
        progress: number;
        bytesUploaded: number;
        totalBytes: number;
        speed: number;
      };
    }
  | {
      error: {
        message: string;
        code: string;
      };
    };

export interface GlobalState {
  alertMessage: string;
  drag: {
    isDragging: boolean;
    paths: string[];
  };
  modal: ModalState;
  files: Array<File>;
  selectedBucket: Selected<Bucket> | undefined;
  activeSelectedBucketId: number | undefined;
  appSetting: AppSettings;
  progress: Record<string, UploadHistory>;
  bucketsRefreshSignal: number;
}

export interface AppSettings {
  id?: number;
  sidebarCollapsed: boolean;
  useSystemProxy: boolean;
  locale: string;
  defaultBucketId: number | undefined;
  lastActiveBucketId: number | undefined;
}

export interface ModalState {
  isShow: boolean;
  children: Snippet | undefined;
  onClose?: () => void;
}

export interface UploadHistory {
  fileId: string;
  filename: string;
  timestamp: number;
  url: string;
  status: UploadStatus;
}

export interface S3Object {
  key: string;
  size: number;
  lastModified: number;
  etag: string;
}

export interface S3ObjectListResponse {
  objects: S3Object[];
  isTruncated: boolean;
  continuationToken?: string;
  totalCount: number;
}

export interface MultipartUpload {
  key: string;
  uploadId: string;
  initiated: number;
}

export interface MultipartUploadListResponse {
  uploads: MultipartUpload[];
  isTruncated: boolean;
  continuationToken?: string;
}
