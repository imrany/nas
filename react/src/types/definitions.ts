export type ErrorBody={
    error_type?:string,
    message:string,
    solution?:any
}

export interface Content{
	root:string,
	path:string,
	name:string,
	metadata:{
		is_file:boolean,
		file_extension:string,
	}
}

export interface Notifications{
	priority:string,
	message:string
}

export interface Folder{
    contents:Content[]
}
