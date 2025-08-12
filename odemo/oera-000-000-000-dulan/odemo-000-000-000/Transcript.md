# Transcript
- **Authors:** [Liam Monninger](liam@ramate.io)

<!-- ltex: enabled=false -->
<!-- vale off -->
00:00 Hey everybody. I'm going to be demonstrating the OIC repository. I'm going to be talking through how to find information, uhm, in it and how to make contributions.
00:10 Uh, the OIC repository is concerned with this thing that I call an artifact. An artifact is a specification, a demonstration like this, run it.
00:24 and other sorts of information which on the whole describe the OIC project. Uhm, so contributions to the repository are going to be made in the form of essentially adding, uh, artifacts or otherwise just doing, uh, chores to keep the repository tidy.
00:43 If you want to learn more about what these artifacts are, uhm, there's actually a glossary artifact which defines that term.
00:52 So, if you go into these, uh, OGLOs, Oglossaries, OGLO0 is defining the term artifact. Okay, so that, that is what the repository is concerned with.
01:10 I'm going to go through finding issues. Choose the repository first, uh, finding things you might be able to contribute to, and then I'm going to hop over and actually, like, working, uh, in an editor with the repo.
01:24 So, uhm, more than likely you're initially going to come to OAC directly to the repository All right, and there's a table for contributors, this contributing.
01:36 Right here, uhm, which has a few different items in it. It has a link to some upcoming, quote-unquote, events, a link to some release candidates, and a link to features and bugs.
01:49 So, the way the project is organized, uh, in terms of tasking, is such that there are these events, things that we want to try and do, push out.
02:00 Uh, engage with people outside the org, or even inside the org, with, at, a certain date. So it's, it's a target that we're shooting for.
02:10 Uh, there are release candidates, which are essentially the things that we are ready to push out to the public, uhm, but, uh, also are needed off-site.
02:24 Often for a particular event. So, you'll have multiple release candidates associated with a particular event. And then you have features and bugs, and these are going to be things, uhm, that are more granular.
02:39 And, uh, there can be children features and children bugs, and there are, uh, a few different labels, uh, involved in this repository to convey semantic meaning and priority about the features and bugs.
02:52 But, uhm, yeah, these are essentially the more granular bits that add up to those release candidates, and then the release candidates are used to perform a particular event.
03:06 So, if you're looking for how to contribute, The best way to do so is using usually gonna be to look at an upcoming event.
03:17 So, in this case, we have this Hello World in Week 0 Readiness event. And then it's gonna be to find the release candidates associated with it.
03:28 So, we have this Week 0 Roadmap, which is a release candidate. I could have also navigated to that release candidate by.
03:37 You know, just looking at the release candidates, but I'm navigating down from the event. And then I'll see essentially all of the issues.
03:46 So, uhm, I have, you know, a roadmap here that I'm referring to for a lot of these issues. And I have this Organize and Update issue.
04:00 and then oh here's the actual demo that I'm working on now. Okay, great. Uhm, you'll see on here that there are links to GitHub Projects.
04:14 I'm using GitHub Projects very lightly just to tag things, and have a bit of a board. I'm not really using it for planning and estimation because I think that's better done.
04:25 By actually getting into the tasks semantically, uhm, that being said, it's, it is useful and I'll show in a bit how, uh, to use the projects and whatnot.
04:38 Before I do that though, I do want to talk about the labels that are assigned to issues, and also to reiterate that structure that I just talked about with events and release candidates.
04:48 Thank Uhm, so if I, if I go check out issues and my different kinds of labels, uhm, you'll see the following.
04:59 You'll see there's a bug label, which is for issues describing a bug that needs to be fixed. There's a delivery label, which refers to something applying to how the software or whatever else in this repository works.
05:13 So it's actually being pushed out to its production environments. There's documentation, docs, uh, events, which is what I talked about before.
05:23 That's that target thing that we are doing, uhm, at a certain point in time. There are features, which are things that add new behavior.
05:33 There are priorities for all of this. So there's priority high, medium, low, and up. Urgent, uhm, essentially this is all about, you know, what you should feel comfortable working on at the expense of something else at a given point in time.
05:46 That's the heuristic to have in mind. Uhm, there are release candidates, which map up in the events. You need release candidates for a particular event to take place.
05:56 And then there's validation. This is things like formal verification, testing, uh, lint, and some various kinds, that sort of thing.
06:04 So these are all the labels that can, can assign to issues. The sort of hierarchy that you should always have in the back of your mind is that there are events that we're targeting, we have release candidates that make those events possible, and then everything else is tagged on, uh, in that structure
06:25 . Okay, so I said that, uhm, you know, you're going to likely come into this project by coming to the repo and then finding events.
06:37 Uhm, all events are mapped back up to, uh, particular, excuse me, to particular events. particular projects.
06:52 So, I'll go into the full scope of, like, organization, uh, projects in the remake video on repo, but there is, uhm, for events, an individual project.
07:10 And And so if you wanted to look at that. Uhm, at the task board instead of exploring down through the issues, uhm, then you can go up into that project and you can work with things that way.
07:22 Uhm, so I am currently working on Odemo Zero. I'm going to move that into in progress. And I'm going to make sure, because it's tagged on multiple projects, to actually update in progress for all of them.
07:37 So in progress, in progress, in progress. Alright, so I am, I'm now working on, uhm, Odemo Zero and making sure to note Cool.
07:51 Uhm, so now I'm going to hop into the actual, uh, editor.
08:01 And work on this repository. I'm working out of cursor, but, uhm, from the shell you should always, uh, open a Nixflake first, the Nixflake associated with the repository.
08:11 Uhm, so I'm going to check out a new branch for the demo. And I'm going to say this is Odemo Zero.
08:21 Uhm, and then Nix develop, uh, in CI, we're using the determinate systems Nix installer.
08:32 I recommend that's what you use locally as well to just create the best parity. Uhm, and then you should, you know, you should see this OAC logo and then you can enter your editor.
08:49 Part of the reason for the Nix shell apart from just generally standardizing dependencies is that there are a lot of pre-commit hooks which will be checked in CI and you should have installed properly.
09:01 Uhm, so yeah, just make sure you have those. I'll show you how that looks in a sec. Alright, so I'm gonna make an edit.
09:11 I'm obviously not gonna have the link to this video yet. bit. To the O-demo. So in general, the structure of this O-demo is- is just wrong.
09:18 It's the- the O-prock structure. So I'm gonna make this O-demo zero and I'm gonna call it, uh, demonstrating the O-S-E repository.
09:30 Okay, cool. I'll make some more changes, uh, in a bit to this structure but, you know, now you have this example.
09:37 Yep. Okay, I'm gonna go ahead and commit the changes so I can show you what the hooks are doing. Umm, uh, feet, uh, add in O-demo zero.
09:47 Okay. These, a bunch of hooks will run. And I'm gonna talk through what each of them is doing. So, I'll start from the top here.
10:09 Umm, there's sort of this, the library approach to the pre-commit hooks. This is the actual hook that gets installed, and then it just tells, umm, you know, the actual hook deck, uh, script when it's run.
10:26 And to go ahead and run each of these hooks in the lib pre-commit, uh, folder. So, therefore, them, the first one is run is footers.
10:37 And footers, essentially, is making sure that this footer template is applied to all non-template markdown throughout the repository. So, that's why you get the logo at the bottom.
10:48 You don't have to format it yourself. Umm. The next hook that's run is the index. So, you'll see that things are indexed by error.
11:02 Umm. So, if you go into a particular artifact, you'll have, uh, this OAC index, which has, as the, uh, error in which a particular artifact was created, and then the, like, description of the artifact, and that is basically just being copied up from the lower level index of the error itself.
11:30 So, like, all the errors for each artifact get indexed here, and then each error has its own index. This means you have to update this manually, uhm, but then the higher order one that combines all the different errors will be updated automatically.
11:49 So that's what, that's what index is doing. Uhm, the third hook that's being run is a links check, which is based on, uh, with jesus lychee.
12:02 Uh, it's checking for broken links, it'll check for broken fragments, uh, as well. Uhm, that sort of thing. And then the last check is a spell check that's based on veil.
12:13 Uhm, which you can see there at the bottom. Okay, so that is, those are the pre-commit hooks. Uhm, unless- If you know what you're doing, I would not fool around with lib pre-commit.
12:31 It's gonna do a lot to help make sure the formatting is strong and give you a chance to fix things before you know you get comments on your PR that aggravate you.
12:44 Uhm, okay. Those are the hooks. One other- What a thing to note in here that will run in CI, which is important, and it helps understand, uh, to understand the repository in the whole, is this labels, um, workflow.
13:07 So, this actually is what creates and deletes, uh, irrelevant labels. Thank you labels. Um, so, if you wanted to make a change to the labels available, you would make it here, and I've done this so that it's, uh, as declarative as possible.
13:27 Um, this structure is also used in a few other places. Okay, so, those are, um, You know, the, like, programmatic bits of this repository at the moment.
13:42 Um, for each of the artifacts, uh, you're gonna want to look individually at what its intent is. So, um, if I take a look at, for example, this description still needs to be updated.
14:00 If I look at, O guides. Uh, I'll see that O guides are OIC guides, uh, which provide useful summaries of OIC.
14:11 So, like, this demo is probably going to be linked from an O guide that you are checking out. Um, in the future, not available right now, there will probably be a key points of contacts for a- contributing to a particular artifact.
14:28 Um, but as long as you're kind of starting from, uh, discussion that's available of some sort and you match the content you're creating to some kind of semantic for one of these artifacts, you should be fine.
14:45 So, you'd go in and, let's say, I was interested- I'm not going to actually commit this, but let's say I was interested in adding a second guide.
14:56 I'm going to give it, uh, a prefix number. Create my readme. And then I'm just going to copy this template over.
15:11 And I would start working, uh, Oh. Oh. Umm, make sure all the pre-commit checks pass, add my PR review, and hopefully someday it gets in.
15:22 So, that's the kind of thing you should expect to be contributing for the most part. It's adding in order to fact under one of these types.
15:28 Okay, so, umm, I think that covers, Uhh, program- programatics and kind of working in this repository.
15:45 Umm, I'm gonna hop back over to the actual GitHub web interface. Okay, so now back in the web interface, as you see, the formatting checks are gonna be running.
15:56 Um, it's just running all those pre-commit hooks, so if it passes locally, it should pass in here. there. Now, I have a memo that I'm gonna write for this, and I'm gonna talk about it more, like I said, in the remade demo, um, for the org, but, uh, in addition to using projects for events, there are
16:16 projects for, like, for example, the repo overall. So when you're creating issues, um, you're gonna wanna tag with as many relevant projects as possible.
16:26 No No now. And update the status as you go along. But there is, for example, like, an OAC, uh, project.
16:34 This board is gonna look, I think, identical to the Ella World in Week Zero Readiness at the moment. Oh, wait a minute.
16:43 It's gonna have, No, no, it's, Uh, it's gonna be missing the Robles, some of the Robles stuff, I think. Umm, but yeah, that's just another way to cut into the project.
16:55 Again, the way that I'm using GitHub projects is, um, just for essentially the views for, um, labeling things that particularly are showing up in, like, a bunch of different repositories, so there's one place to look at it.
17:12 The real important, like, information about, the task, um, its priority, um, and status, I think should be gleaned from actually reading through the issues and tracing out that hierarchy from events.
17:27 Um, so, you know, that's, again, to re-emphasize that going in and finding these events is the best way to, to understand what the project is currently looking at.
17:44 Okay, so let's go and see. Pull request. CI is still running, so I'll give it a sec longer and just show it's green for the end of the video.
17:54 Okay, literally right after I click pause on them. But, um, yeah, so CI will run. Um, this is obviously not yet done.
18:01 I would usually mark this as a- draft with, um, you know, other reviewers in place, but I didn't hear. So, you know, that's- that's- that's- will be checked for the moment of those pre-commit hooks.
18:14 It'll make sure that the format is nice. Um, you haven't made too many obvious errors. Okay, so that's using the OIC repo to provide some high-level notes.
18:25 The OIC repo is all about these- artifacts, um, when you're actually working on adding artifacts, contributing artifacts, uh, make sure you're working out of the nickshell.
18:38 It will, um, take steps to ensure proper formatting, um, and save a lot of headaches. Um, if you're trying to figure out how to- We'll in the one.
18:51 Find what you might want to work on, what you should add. Umm, you'll want to track down from the events.
18:58 Umm, and there's gonna be a sort of hierarchy between events, release candidates and other issues, whereby, um, issues generally will be mapped to a release candidate and release candidates.
19:15 It's will be mapped to a particular event. Umm, finally, if you want to have a task board view, umm, you can go ahead and use the get a projects page.
19:26 Umm, to make it so that other people have that available, please do go ahead and tag with relevant projects as you work on anything.
19:38 That's all. See you in the next. Next.
<!-- vale on -->
<!-- ltex: enabled=true -->

<!--OAC FOOTER: DO NOT REMOVE THIS LINE-->
---

<div align="center">
  <a href="https://github.com/ramate-io/oac">
    <picture>
      <source srcset="/assets/oac-inverted-transparent.png" media="(prefers-color-scheme: dark)">
      <img height="24" src="/assets/oac-transparent.png" alt="OAC"/>
    </picture>
  </a>
  <br/>
  <sub>
    <b>Ordered Atomic Collaboration (OAC)</b>
    <br/>
    &copy; 2025 <a href="https://github.com/ramate-io/oac">ramate-io/oac</a>
    <br/>
    <a href="https://github.com/ramate-io/oac/blob/main/LICENSE">MIT License</a>
    <br/>
    <a href="https://www.ramate.io">ramate.io</a>
  </sub>
</div>
