export interface JiraVersionPostData {
	archived: boolean;
	description: string;
	name: string;
	projectId: number;
	released: boolean;
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
