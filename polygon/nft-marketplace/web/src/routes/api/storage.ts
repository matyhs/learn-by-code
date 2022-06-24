import { NFTStorage, File } from 'nft.storage';
import type { TokenInput } from 'nft.storage/dist/src/token';
import type { IPFSMetadata } from '$lib/types/type';

const API_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDQ5NzhDQzFDOEJmREVBMUJCQTBCMTZFMDczNTFiMGM2M0RkODA0NkIiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY1NTAzMjgzMjk5NCwibmFtZSI6Inh5el9ta3QifQ.rToVrotmBzfSUNCYtj-93FXGcK8d4ztVLBT7QzQC-uE";

export const post = async (input: {
                            request: Request
                        }): Promise<Response> => 
{
    const data = await input.request.formData();
    const nameData = await data.get('name');
    const descriptionData = await data.get('description');
    const uploadData = await data.get('image');
    
    if (uploadData == null || nameData == null || descriptionData == null) {
        const init = {
            "status": 500
        };

        return new Response("", init);
    }
    
    const name = nameData.valueOf() as string;
    const description = descriptionData.valueOf() as string;
    const file: File = uploadData.valueOf() as File;
    const buffer = await file.arrayBuffer();
    const image = new File([buffer], file.name, { type: file.type });
    const nft: TokenInput = {
        image,
        name,
        description
    }

    const client = new NFTStorage( { token: API_KEY });
    const metadata = await client.store(nft);

    if (metadata == null) {
        const init = {
            "status": 500
        };
        return new Response("", init);
    }
    
    const init = {
        "status": 200
    };
    const body : IPFSMetadata = { url: metadata.url };
    return new Response(JSON.stringify(body), init);
}