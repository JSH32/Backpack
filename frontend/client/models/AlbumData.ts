/* istanbul ignore file */
/* tslint:disable */

export type AlbumData = {
    /**
     * Date of album creation
     */
    created: string;
    /**
     * Optional album description
     */
    description?: string;
    id: string;
    name: string;
    /**
     * Is the album public.
     */
    public: boolean;
    /**
     * User who created the album.
     */
    userId: string;
};

