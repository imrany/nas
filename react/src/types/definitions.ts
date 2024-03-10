export type ErrorBody={
    error_type?:string,
    message:string,
    solution?:any
}

export interface Folder{
    contents:{
        root:string,
        path:string,
        name:string,
        metadata:{
            is_file:boolean,
            file_extension:string
        }
    }[]
}