export interface Project {
	id?: number;
	key: string;
	name: string;
	roles?: string[];
	issues?: Record<string, unknown>[];
}

export type Projects = Project[];
