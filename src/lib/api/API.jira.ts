// import { PUBLIC_JIRA_API_KEY, PUBLIC_JIRA_API_URL, PUBLIC_JIRA_API_USER } from '$env/static/public';

// const headers = new Headers();
// headers.append('content-type', 'application/json');

// export async function createRelease(name, description, project) {
// 	const options = {
// 		method: 'POST',
// 		headers
// 	};
// 	const body = JSON.stringify({
// 		archived: false,
// 		description,
// 		name,
// 		project,
// 		released: false
// 	});

// 	try {
// 		// Make the API POST call
// 		const response = await fetch(PUBLIC_JIRA_API_URL, {
// 			...options,
// 			body
// 		});

// 		if (response.ok) {
// 			return 'success'; // Return 'success' if the API call succeeds
// 		} else {
// 			throw new Error('API call failed');
// 		}
// 	} catch (error) {
// 		console.error('Error:', error);
// 		throw error; // Rethrow the error
// 	}
// }

// export async function getUserProjects() {
// 	const authString = `${PUBLIC_JIRA_API_KEY}:${PUBLIC_JIRA_API_USER}`;
// 	const encodedAuth = btoa(authString);

// 	headers.append('Authorization', `Basic ${encodedAuth}`);
// 	const options = {
// 		method: 'GET',
// 		headers
// 	};

// 	try {
// 		// Make the API POST call
// 		const response = await fetch(PUBLIC_JIRA_API_URL, {
// 			...options
// 		});

// 		if (response.ok) {
// 			console.log('🚀 ~ getUserProjects ~ response:', response);
// 			return response.json(); // Return 'success' if the API call succeeds
// 		} else {
// 			throw new Error('API call failed');
// 		}
// 	} catch (error) {
// 		console.error('Error:', error);
// 		throw error; // Rethrow the error
// 	}
// }
