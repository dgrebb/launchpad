import type { Projects } from '$lib/types';
import mockProjects from '$lib/../test/mocks/jira/user-projects.json';

const createProjectsState = () => {
	let projects: Projects = $state([]);

	return {
		setProjects: (data: Projects) => {
			projects = data;
		},
		getProjects: () => {
			return mockProjects;
		}
	};
};

export const projectsState = createProjectsState();
