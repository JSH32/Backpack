/* istanbul ignore file */
/* tslint:disable */

/**
 * Standard message response.
 */
export type MessageResponse = {
    data: any;
    /**
     * Optional error (only on 500 errors)
     */
    error?: string;
    /**
     * Message
     */
    message: string;
};

