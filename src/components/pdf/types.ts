import type { SplitOptions } from './SplitSettings.vue'
import type { CompressionLevel } from './CompressSettings.vue'

export interface PDFSettings {
  splitOptions: SplitOptions
  compressionLevel: CompressionLevel
  deletePages: string
  outputPath: string
}