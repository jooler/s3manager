import { globalState } from "./store.svelte";

export let selectedLocale = $state("en");

export let en = $state({
  addBucket: {
    invalidS3Api: "Invalid S3 API format, please refer to help",
    howToUse: "How to use",
    use: `# Cloudflare R2 Configuration Guide

## Connection Setup

1. Find your Cloudflare R2 S3 API URL:
   - Go to your R2 bucket settings page
   - Look for the "S3 API" section under "Bucket Details"
   - Copy the S3 API URL in the following format:
   \`\`\`
   https://[accountId].r2.cloudflarestorage.com/[bucketName]
   \`\`\`
   The application will automatically extract the \`Bucket Name\` and \`Account ID\` from this URL.

2. Create an API token with the following steps:
   - Go to Cloudflare R2 API Tokens page: https://dash.cloudflare.com/?to=/:account/r2/api-tokens
   - Generate a new token with at least read and write permissions for objects
   - Copy the "Access Key ID" and paste it into the "Access Key" field
   - Copy the "Secret Access Key" and paste it into the "Secret Key" field

3. (Optional) Configure a Custom Domain:
   - If you have a custom domain for your R2 bucket, enter it in the "Custom Domain" field
   - This domain will be used when generating shareable links for uploaded files

## Verification and Save

1. Click the "Check" button to verify your bucket connectivity
2. If the connection test passes, click "Save" to store your configuration`,
    title: "Add Cloudflare R2 Bucket",
    titleR2: "Add Cloudflare R2 Bucket",
    titleOSS: "Add Aliyun OSS Bucket",
    selectBucketType: "Select Storage Type",
    cancel: "Cancel",
    save: "Save",
    addNew: "Add New Bucket",
    requiredField: "This field is required",
    check: "Check",
    checking: "Checking...",
    labels: {
      s3Api: "S3 API",
      bucketName: "Bucket Name",
      accountId: "Account ID",
      accessKey: "Access Key",
      secretKey: "Secret Key",
      customDomain: "Custom Domain, e.g. https://example.com",
      endpoint: "Endpoint (for S3-compatible services)",
    },
  },
  common: {
    upload: "Upload",
    setting: "Setting",
    transfer: "Transfer",
    buckets: "Buckets",
    collapse: "Collapse Sidebar",
    expand: "Expand Sidebar",
    loading: "loading...",
    close: "close",
    uploading: "Uploading...",
    delete: "Delete",
    useSystemProxy: "Use system proxy",
    language: "Language",
    selectLanguage: "Select language",
    clipboardReadError: "Failed to read clipboard content",
    uploadError: "Upload failed, please try again",
    noBucketWarning: "Please add a bucket in settings first",
  },
  settings: {
    buckets: "Buckets",
    language: "Language",
    bucketDetails: {
      bucket: "Bucket",
      accountId: "Account ID",
    },
    defaultBucket: "Default Bucket",
    setDefault: "Set as Default",
    edit: "Edit",
  },
  uploadTargetSelector: {
    title: "Bucket",
    placeholder: "Select Bucket",
  },
  textUploader: {
    textPlaceholder: "Enter text content to upload",
    filenamePlaceholder: "Enter remote filename",
    textSeparator: "Text Content",
    filenameSeparator: "Remote Filename",
  },
  tabSwitcher: {
    file: "Upload File",
    folder: "Upload Folder",
    text: "Upload Text",
    clipboard: "Clipboard",
  },
  fileUploader: {
    preview: {
      previewFailed: "Preview failed",
      filePreview: "File Preview",
      filename: "Filename:",
      remotePath: "Remote Path:",
    },
    previewFailed: "Preview failed",
    uploadSuccess: "Upload successful",
    uploadFailed: "Upload failed, please try again",
    filePreview: "File preview",
    filename: "Filename",
    remotePath: "Remote path",
    bucketReady: "Your bucket is ready",
    dragDrop: "Drag and drop or",
    clickToSelect: "click to select files",
    globalPath: "Global path",
    remotePathPlaceholder: "Remote path",
    filenamePlaceholder: "Remote filename",
    selectFile: "Select File",
    selectFolder: "Select Folder",
    selectClipboard: "Select Clipboard",
    selectNewText: "New Text",
    bucketReadyTip: "Your bucket is ready, drag and drop files here, or:",
    uploadStatus: {
      nothing: "Nothing",
      uploadComplete: "Upload Complete",
      uploadFailed: "Upload Failed:",
      cancelled: "Cancelled",
      waiting: "Waiting...",
      copySuccess: "Copy Success",
      copyFailed: "Copy Failed",
      speed: "MB/s",
      total: "Total",
      page: "Page",
      previous: "Previous",
      next: "Next",
      cancel: "Cancel",
    },
    upload: {
      globalPath: "Global path",
      clear: "Clear",
      upload: "Upload",
      uploading: "Uploading...",
      remotePath: "Remote path",
      remoteFilename: "Remote filename",
      uploadFailed: "Upload failed, please try again",
    },
  },
  fileDrag: {
    dragHere: "Drag files here",
    releaseToConfirm: "Release to confirm",
  },
  clipboardUploader: {
    refresh: "Refresh clipboard content",
    clipboardText: "Clipboard text",
    clipboardHtml: "Clipboard HTML",
    clipboardImage: "Clipboard image",
    clipboardRtf: "Clipboard RTF",
    clipboardFile: "Clipboard file",
  },
  alert: {
    uploadSuccess: "Upload successful",
    uploadFailed: "Upload failed, please try again",
    getStatusFailed: "Failed to get upload status",
    allFilesUploaded: "All files uploaded",
  },
  tools: {
    getFileDetailsFailed: "Failed to get file details",
  },
  transfer: {
    title: "Transfer",
    tabs: {
      all: "All",
      inProgress: "In Progress",
      completed: "Completed",
    },
  },
  manage: {
    title: "Manage",
    toolbar: {
      pageSize: "Items per page",
      refresh: "Refresh",
    },
    files: {
      name: "Name",
      size: "Size",
      modified: "Modified",
      actions: "Actions",
      preview: "Preview",
      copyUrl: "Copy URL",
      download: "Download",
      delete: "Delete",
      deleteConfirm: "Are you sure you want to delete this file?",
      deleteSuccess: "File deleted successfully",
      deleteFailed: "Failed to delete file",
      copySuccess: "URL copied to clipboard",
      copyFailed: "Failed to copy URL",
      noFiles: "No files in bucket",
    },
    multipartUploads: {
      title: "In-Progress Multipart Uploads",
      name: "Name",
      initiated: "Initiated",
      actions: "Actions",
      abort: "Abort",
      abortConfirm: "Are you sure you want to abort this upload?",
      abortSuccess: "Upload aborted successfully",
      abortFailed: "Failed to abort upload",
      noUploads: "No in-progress uploads",
    },
    pagination: {
      page: "Page",
      of: "of",
      previous: "Previous",
      next: "Next",
    },
  },
});

export let zh = $state({
  addBucket: {
    invalidS3Api: "S3 API 格式不正确，请参阅帮助",
    howToUse: "如何使用",
    use: `# Cloudflare R2 配置指南

## 连接设置

1. 获取 Cloudflare R2 S3 API URL：
   - 进入 R2 存储桶的设置页面
   - 在"存储桶详细信息"中找到"S3 API"部分
   - 复制 S3 API URL，格式如下：
   \`\`\`
   https://[accountId].r2.cloudflarestorage.com/[bucketName]
   \`\`\`
   程序将自动从此 URL 中提取 \`Bucket Name\` 和 \`Account ID\`。

2. 创建 API 令牌，步骤如下：
   - 前往 Cloudflare R2 API 令牌页面：https://dash.cloudflare.com/?to=/:account/r2/api-tokens
   - 创建新的令牌，确保至少具有对象的读写权限
   - 复制"访问密钥 ID"并粘贴到"Access Key"字段
   - 复制"机密访问密钥"并粘贴到"Secret Key"字段

3. （可选）配置自定义域名：
   - 如果您为 R2 存储桶配置了自定义域名，请在"Custom Domain"字段中输入
   - 上传文件后生成的分享链接将使用此域名

## 验证与保存

1. 点击"Check"按钮验证存储桶连接
2. 连接测试通过后，点击"Save"保存配置`,
    title: "添加 Cloudflare R2 存储桶",
    titleR2: "添加 Cloudflare R2 存储桶",
    titleOSS: "添加阿里云 OSS 存储桶",
    selectBucketType: "选择存储类型",
    cancel: "取消",
    save: "保存",
    addNew: "添加新存储桶",
    check: "检查",
    checking: "检查中...",
    labels: {
      s3Api: "S3 API",
      bucketName: "Bucket 名称",
      accountId: "Account ID",
      accessKey: "Access Key",
      secretKey: "Secret Key",
      customDomain: "自定义域名，例如 https://example.com",
      endpoint: "Endpoint（用于 S3 兼容服务）",
    },
  },
  common: {
    upload: "上传",
    setting: "设置",
    transfer: "传输",
    buckets: "存储桶",
    collapse: "收起侧边栏",
    expand: "展开侧边栏",
    loading: "加载中...",
    close: "关闭",
    uploading: "上传中...",
    delete: "删除",
    useSystemProxy: "使用系统代理",
    language: "语言",
    selectLanguage: "选择语言",
    clipboardReadError: "读取剪贴板内容失败",
    uploadError: "上传失败，请重试",
    noBucketWarning: "请先在设置中添加存储桶",
  },
  settings: {
    buckets: "存储桶",
    language: "语言",
    bucketDetails: {
      bucket: "存储桶",
      accountId: "账户 ID",
    },
    defaultBucket: "默认存储桶",
    setDefault: "设为默认",
    edit: "编辑",
  },
  uploadTargetSelector: {
    title: "存储桶",
    placeholder: "选择存储桶",
  },
  textUploader: {
    textPlaceholder: "输入要上传的文本内容",
    filenamePlaceholder: "输入远程文件名",
    textSeparator: "文本内容",
    filenameSeparator: "远程文件名",
  },
  tabSwitcher: {
    file: "上传文件",
    folder: "上传文件夹",
    text: "上传文本",
    clipboard: "剪贴板",
  },
  fileUploader: {
    preview: {
      previewFailed: "预览失败",
      filePreview: "文件预览",
      filename: "文件名：",
      remotePath: "远程路径：",
    },
    previewFailed: "预览失败",
    uploadSuccess: "上传成功",
    uploadFailed: "上传失败，请重试",
    filePreview: "文件预览",
    filename: "文件名",
    remotePath: "远程路径",
    bucketReady: "您的存储桶已就绪",
    dragDrop: "拖放或",
    clickToSelect: "点击选择文件",
    globalPath: "全局路径",
    remotePathPlaceholder: "远程路径",
    filenamePlaceholder: "远程文件名",
    selectFile: "选择文件",
    selectFolder: "选择文件夹",
    selectClipboard: "选择剪贴板",
    selectNewText: "选择新建文本",
    bucketReadyTip: "您的存储桶已就绪，拖放文件到此，或：",
    uploadStatus: {
      nothing: "暂无内容",
      uploadComplete: "上传完成",
      uploadFailed: "上传失败：",
      cancelled: "已取消",
      waiting: "等待中...",
      copySuccess: "复制成功",
      copyFailed: "复制失败",
      speed: "MB/s",
      total: "总数",
      page: "页码",
      previous: "上一页",
      next: "下一页",
      cancel: "取消",
    },
    upload: {
      globalPath: "全局路径",
      clear: "清空",
      upload: "上传",
      uploading: "上传中...",
      remotePath: "远程路径",
      remoteFilename: "远程文件名",
      uploadFailed: "上传失败，请重试",
    },
  },
  fileDrag: {
    dragHere: "拖动文件到此",
    releaseToConfirm: "松手即可确认",
  },
  clipboardUploader: {
    refresh: "刷新剪贴板内容",
    clipboardText: "剪贴板文本",
    clipboardHtml: "剪贴板 HTML",
    clipboardImage: "剪贴板图片",
    clipboardRtf: "剪贴板 RTF",
    clipboardFile: "剪贴板文件",
  },
  alert: {
    uploadSuccess: "上传成功",
    uploadFailed: "上传失败，请重试",
    getStatusFailed: "获取上传状态失败",
    allFilesUploaded: "所有文件都上传完成",
  },
  tools: {
    getFileDetailsFailed: "获取文件详情失败",
  },
  transfer: {
    title: "传输",
    tabs: {
      all: "全部",
      inProgress: "传输中",
      completed: "已完成",
    },
  },
  manage: {
    title: "管理",
    toolbar: {
      pageSize: "每页项数",
      refresh: "刷新",
    },
    files: {
      name: "名称",
      size: "大小",
      modified: "修改时间",
      actions: "操作",
      preview: "预览",
      copyUrl: "复制 URL",
      download: "下载",
      delete: "删除",
      deleteConfirm: "确定要删除此文件吗？",
      deleteSuccess: "文件删除成功",
      deleteFailed: "删除文件失败",
      copySuccess: "URL 已复制到剪贴板",
      copyFailed: "复制 URL 失败",
      noFiles: "存储桶中没有文件",
    },
    multipartUploads: {
      title: "正在进行的多部分上传",
      name: "名称",
      initiated: "开始时间",
      actions: "操作",
      abort: "取消",
      abortConfirm: "确定要取消此上传吗？",
      abortSuccess: "上传已取消",
      abortFailed: "取消上传失败",
      noUploads: "没有正在进行的上传",
    },
    pagination: {
      page: "页码",
      of: "共",
      previous: "上一页",
      next: "下一页",
    },
  },
});

export function t() {
  return globalState.appSetting.locale === "en" ? en : zh;
}
