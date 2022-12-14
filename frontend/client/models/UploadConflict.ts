/* istanbul ignore file */
/* tslint:disable */

import type { UploadData } from './UploadData';

/**
 * Identical file was already uploaded.
 */
export type UploadConflict = {
    message: string;
    upload: UploadData;
};

