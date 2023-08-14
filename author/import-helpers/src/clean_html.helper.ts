
import TurndownService from 'turndown';

export const hb_clean_html = (s: string) : string =>  {

    const turndownService: any = new TurndownService();

    let md = turndownService.turndown(s);

    return md;
}

