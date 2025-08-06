/* istanbul ignore file */
/* tslint:disable */

export type Page_AlbumData = {
    items: Array<{
        /**
         * Date of album creation
         */
        created: string;
        /**
         * Optional album description
         */
        description?: string | null;
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
    }>;
    page: number;
    pages: number;
};

