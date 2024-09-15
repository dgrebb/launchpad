export interface ProjectInfo {
	id: string;
	key: string;
	name: string;
	avatar_url: string;
}

export type Projects = ProjectInfo[];

export interface ProjectWithSelection extends ProjectInfo {
	selected: boolean | 'indeterminate';
}
