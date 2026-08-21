use async_graphql::Enum;

pub trait SearchSort {
    fn sort_key(&self) -> &str;
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SearchAnimeSort {
    TitleRomaji,
    TitleRomajiDesc,
    Year,
    YearDesc,
    Season,
    SeasonDesc,
    CreatedAtDesc,
}

impl SearchSort for SearchAnimeSort {
    fn sort_key(&self) -> &str {
        match self {
            SearchAnimeSort::TitleRomaji => "title:asc",
            SearchAnimeSort::TitleRomajiDesc => "title:desc",
            SearchAnimeSort::Year => "year:asc",
            SearchAnimeSort::YearDesc => "year:desc",
            SearchAnimeSort::Season => "season:asc",
            SearchAnimeSort::SeasonDesc => "season:desc",
            SearchAnimeSort::CreatedAtDesc => "created_at:desc",
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SearchArtistSort {
    NameMain,
    NameMainDesc,
    CreatedAtDesc,
}

impl SearchSort for SearchArtistSort {
    fn sort_key(&self) -> &str {
        match self {
            SearchArtistSort::NameMain => "name:asc",
            SearchArtistSort::NameMainDesc => "name:desc",
            SearchArtistSort::CreatedAtDesc => "created_at:desc",
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SearchPlaylistSort {
    Name,
    NameDesc,
    CreatedAtDesc,
}

impl SearchSort for SearchPlaylistSort {
    fn sort_key(&self) -> &str {
        match self {
            SearchPlaylistSort::Name => "name:asc",
            SearchPlaylistSort::NameDesc => "name:desc",
            SearchPlaylistSort::CreatedAtDesc => "created_at:desc",
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SearchSeriesSort {
    TitleRomaji,
    TitleRomajiDesc,
    CreatedAtDesc,
}

impl SearchSort for SearchSeriesSort {
    fn sort_key(&self) -> &str {
        match self {
            SearchSeriesSort::TitleRomaji => "title:asc",
            SearchSeriesSort::TitleRomajiDesc => "title:desc",
            SearchSeriesSort::CreatedAtDesc => "created_at:desc",
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SearchStudioSort {
    Name,
    NameDesc,
    CreatedAtDesc,
}

impl SearchSort for SearchStudioSort {
    fn sort_key(&self) -> &str {
        match self {
            SearchStudioSort::Name => "name:asc",
            SearchStudioSort::NameDesc => "name:desc",
            SearchStudioSort::CreatedAtDesc => "created_at:desc",
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SearchAnimeThemeSort {
    SongTitleRomaji,
    SongTitleRomajiDesc,
    AnimeYear,
    AnimeYearDesc,
    AnimeSeason,
    AnimeSeasonDesc,
    CreatedAtDesc,
}

impl SearchSort for SearchAnimeThemeSort {
    fn sort_key(&self) -> &str {
        match self {
            SearchAnimeThemeSort::SongTitleRomaji => "song_title:asc",
            SearchAnimeThemeSort::SongTitleRomajiDesc => "song_title:desc",
            SearchAnimeThemeSort::AnimeYear => "anime.year:asc",
            SearchAnimeThemeSort::AnimeYearDesc => "anime.year:desc",
            SearchAnimeThemeSort::AnimeSeason => "anime.season:asc",
            SearchAnimeThemeSort::AnimeSeasonDesc => "anime.season:desc",
            SearchAnimeThemeSort::CreatedAtDesc => "created_at:desc",
        }
    }
}
