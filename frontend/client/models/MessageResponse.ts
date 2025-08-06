/* istanbul ignore file */
/* tslint:disable */

/**
 * Standard message response.
 *
 * Usually the only field will be `message`
 */
export type MessageResponse = {
    /**
     * Optional data, can be any JSON object
     */
    data?: any;
    /**
     * Optional error (only on 500 errors)
     */
    error?: string | null;
    /**
     * Message
     */
    message: string;
};

