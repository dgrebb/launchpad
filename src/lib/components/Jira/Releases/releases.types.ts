export interface JiraVersionPostData {
	archived?: boolean;
	description?: string | null | undefined | number | string[];
	name: string;
	projectId: number;
	releaseDate?: string;
	startDate?: string;
	released?: boolean;
}

export interface JiraVersionResponseData {
	self: string;
	id: string;
	description: string;
	name: string;
	archived: boolean;
	released: boolean;
	projectId: number;
}
