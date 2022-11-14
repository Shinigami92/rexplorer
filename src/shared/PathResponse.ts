export interface PathResponse {
  /**
   * The full path.
   */
  readonly path: string;

  /**
   * The name of the file or folder.
   */
  readonly name: string;

  /**
   * The size of the file in bytes.
   */
  readonly size: number;

  /**
   * Whether the path is a file or a folder.
   */
  readonly is_dir: boolean;
}
