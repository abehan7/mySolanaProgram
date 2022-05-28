// 아~ program이 메인이고 아래 derive(Accounts)이거가 여기서 사용하는 참조키들인듯
// Accounts/ Default / Errors 이거는 솔라나 내부적으로 사용되는 변수들인듯
// Context이거랑 ProgramResult이것도
#[program]
pub mod my_solana_program {
    use super::*;
    // setup_platform 이거는 construct같은거인듯
    // Context이거를 통해서 참조해 오는 듯
    // ProgramResult 이거는 return느낌
    // #1 fn
    pub fn setup_platform(ctx: Context<TweetPlatform>) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;
        tweet.likes = 0;
        tweet.message = ("").to_string();
        Ok(())
    }

    // #2 fn
    pub fn write_tweet(
        ctx: Context<WriteTweet>,
        message: String,
        user_public_key: Pubkey
    ) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;

        if !tweet.message.trim().is_empty() {
            return Err(Errors::CannotUpdateTweet.into());
        }

        if message.trim().is_empty() {
            return Err(Errors::EmtpyMessage.into());
        }

        tweet.message = message;
        tweet.likes = 0;
        tweet.creator = user_public_key;

        Ok(())
    }

    // #3 fn
    pub fn like_tweet(ctx: Context<LikeTweet>, user_liking_tweet: Pubkey) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;

        if tweet.message.trim().is_empty() {
            return Err(Errors::NotValidTweet.into());
        }

        if tweet.likes == 5 {
            return Err(Errors::ReachedMaxLikes.into());
        }

        let mut iter = tweet.people_who_liked.iter();
        if iter.any(|&v| v == user_liking_tweet) {
            return Err(Errors::UserLikedTweet.into());
        }

        tweet.likes += 1;
        tweet.people_who_liked.push(user_liking_tweet);

        Ok(())
    }
}
//FIXME:  referances
#[derive(Accounts)]
pub struct TweetPlatform<'info> {
    #[account(init, payer = user, space = 9000 )]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WriteTweet<'info> {
    #[account(mut)]
    // ctx.accounts.tweet
    pub tweet: Account<'info, Tweet>,
}

#[derive(Accounts)]
pub struct LikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct Tweet {
    message: String,
    likes: u8,
    creator: Pubkey,
    people_who_liked: Vec<Pubkey>, // with  #[derive(Default)] we can assign default values
}

//FIXME: ERROR TREATMENT
// 여기는 그냥 에러 처리
#[error]
pub enum Errors {
    #[msg("Tweet message cannot be updated")]
    CannotUpdateTweet,

    #[msg("Message cannot be empty")]
    EmtpyMessage,

    #[msg("Cannot receive more than 5 likes")]
    ReachedMaxLikes,

    #[msg("Cannot like a tweet without a valid message")]
    NotValidTweet,

    #[msg("User has already liked the tweet")]
    UserLikedTweet,
}