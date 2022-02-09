use super::{ Blokka, State, Render, TableHeaderItem, SelectableList, Index, selectable_table, get_percentage_width };
use mpd_client::{ Client, commands, commands::responses::SongInQueue };
use tui::{ backend::Backend, layout::Rect, Frame };
use anyhow::Result;

pub struct Queue {
    pub index: Index,
    pub songs: Vec<SongInQueue>,
}

impl Queue {
    pub async fn new(client: Client) -> Result<Self> {
        let songs = client.command(commands::Queue).await?;

        Ok(Self {
            index: Index::new(songs.len()),
            songs,
        })
    }

    pub async fn play(&self, client: Client, index: usize) {
        let song = self.songs.get(index).unwrap().id;
        client.command(commands::Play::song(song)).await.unwrap();
    }
}

impl<B: Backend> Render<B> for Queue {
    fn render(&self, f: &mut Frame<B>, state: &State<B>, layout_chunk: Rect) {
        let highlight_state = (
            state.blocks.is_active(Blokka::Main),
            state.blocks.is_hovered(Blokka::Main)
        );

        let items = self.songs
            .iter()
            .map(|song| { 
                let artists = song.song.artists();
                let artist = if artists.len() > 0 {
                    artists[0].to_string()
                } else {
                    String::new()
                };         

                //if title is empty, take file name

                vec![
                    song.position.0.to_string(), 
                    song.song.title().unwrap_or("none").to_string(), 
                    artist,
                    song.song.duration.unwrap_or(std::time::Duration::from_secs(1)).as_secs().to_string()
                ]
            }).collect::<Vec<Vec<String>>>();


        let header =  vec![
            TableHeaderItem { text: "#", width: 3 },
            TableHeaderItem { text: "Title", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 5 },
            TableHeaderItem { text: "Artist", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) },
            // TableHeaderItem { text: "Album", width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) },
            TableHeaderItem { text: "Length", width: get_percentage_width(layout_chunk.width, 1.0 / 5.0) },
        ];

        selectable_table(
            f,
            state,
            layout_chunk,
            " Queue ",
            &header,
            items,
            self.index.inner,
            highlight_state,
        )
    }
}

impl SelectableList for Queue {
    fn index(&mut self) -> &mut Index {
        &mut self.index
    }
}