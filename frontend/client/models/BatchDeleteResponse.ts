/* istanbul ignore file */
/* tslint:disable */

import type { BatchFileError } from './BatchFileError';

/**
 * Response containing information about deleted files.
 */
export type BatchDeleteResponse = {
    /**
     * All successfully deleted files.
     */
    deleted: Array<string>;
    /**
     * Errors for all failed deletions.
     */
    errors: Array<BatchFileError>;
};

