import type { SplitOptions } from './SplitSettings.vue'
import type { CompressionLevel } from './CompressSettings.vue'
import type { MergeOptions } from './MergeSettings.vue'

export interface PDFSettings {
  splitOptions: SplitOptions
  compressionLevel: CompressionLevel
  deletePages: string
  mergeOptions: MergeOptions
  outputPath: string
}