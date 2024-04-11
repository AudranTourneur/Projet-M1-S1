import { PUBLIC_API_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';


export const POST: RequestHandler = async ({ params, fetch }) => {
    const name = params.id;
    const nameChanged = params.currentlyEditingValueName;
    const response = await fetch(`${PUBLIC_API_URL}/volume/${name}/changeName`, {
        method: 'POST',
      });
    
      if (!response.ok) {
        const errorMessage = `Failed to change volume name ${name}`;
        throw new Error(errorMessage);
      }
    
      return json({ message: `Volume ${name} changed name.` });
    };