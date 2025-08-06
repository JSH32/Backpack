/* istanbul ignore file */
/* tslint:disable */

export type UploadData = {
    hash: string;
    id: string;
    name: string;
    originalName: string;
    public: boolean;
    size: number;
    thumbnailUrl?: string | null;
    uploaded: number;
    uploader: string;
    url?: string | null;
};

