import TurndownService from 'turndown';

const hb_clean_html = (s: string) : string =>  {

    const turndownService: any = new TurndownService();

    let md = turndownService.turndown(s);

    return md;
}

// uncomment in build file 
// handlebars.registerHelper('clean_html', hb_clean_html);
