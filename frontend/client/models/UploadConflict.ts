/* istanbul ignore file */
/* tslint:disable */

import type { FileData } from './FileData';

/**
 * Identical file was already uploaded.
 */
export type UploadConflict = {
    file: FileData;
    message: string;
};

