import type { Projects } from '@types';

export const createProjectsState = () => {
	let projectsState: Projects = $state([]);

	return {
		setProjects: (projects: Projects) => {
			projectsState = projects;
		},
		getProjects: () => {
			return projectsState;
		}
	};
};

export const projectsState = createProjectsState();
