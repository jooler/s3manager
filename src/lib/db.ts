import Dexie from "dexie";
import type { AppSettings, Bucket, UploadHistory } from "./type";

class AppDatabase extends Dexie {
  buckets!: Dexie.Table<Bucket, number>;
  history!: Dexie.Table<UploadHistory, number>;
  appSettings!: Dexie.Table<AppSettings, number>;

  constructor() {
    super("AppDatabase");
    this.version(7).stores({
      buckets: "++id, bucketName",
      history: "fileId, timestamp, status",
      appSettings: "++id",
    });
  }
}

const db = new AppDatabase();

export default db;
