// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SoundMod
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingIrrKlang;
// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  [StandardModule]
  pub sealed class SoundMod
  {
    pub static LastSong: String;
     static ISoundEngine engine;
     static ISoundSource medley;
     static ISound eventsound;
     static ISound eventsoundbg;
    pub static bool NOSOUND;
    pub static bool SYSTEMFAILURE = false;
    pub static dssSoundCounter: i32;
    pub static let mut dssSoundMaxCounter: i32 = 9;
     static ISound[] dssSoundRef = new ISound[10];
     static int[] dssStatus = new int[10];
     static int[] dssType = new int[10];
     static int[] dssId = new int[10];
     static int[] dssMsMark = new int[10];
     static DateTime startDatetime;
    pub static let mut DSS_FADE_MS: i32 = 5000;
    pub static let mut DSS_MARGIN_MS: i32 = 300;

    pub static void InitSound(Form FormHandle)
    {
      try
      {
        SoundMod.engine = ISoundEngine::new();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        SoundMod.NOSOUND = true;
        SoundMod.SYSTEMFAILURE = true;
        ProjectData.ClearProjectError();
      }
      SoundMod.dssSoundCounter = -1;
      SoundMod.startDatetime = DateAndTime.Now;
    }

    pub static void PlayAWave(
      Soundfile: String,
       teditobj: EditClass,
      bool overrule = false,
      let mut volumeMod: i32 = 0)
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE)
        return;
      str: String = "";
      if (!Information.IsNothing( DrawMod.TGame) && !Information.IsNothing( DrawMod.TGame.Data.SoundDir))
        str = DrawMod.TGame.Data.SoundDir;
      if (Operators.CompareString(str, "", false) == 0)
        str = DrawMod.TGame.ModSoundDir;
      if (!Information.IsNothing( DrawMod.TGame))
        Soundfile = Soundfile.Replace("sound", str);
      if (!Information.IsNothing( SoundMod.eventsound) && SoundMod.eventsound.Finished)
      {
        SoundMod.eventsound.Dispose();
        SoundMod.eventsound = (ISound) null;
      }
      if (!(teditobj.SoundOn | overrule) || !File.Exists(Soundfile))
        return;
      SoundMod.eventsound = SoundMod.engine.Play2D(Soundfile, false);
      if (Information.IsNothing( SoundMod.eventsound))
        return;
      if (volumeMod > 0)
        SoundMod.eventsound.Volume = Math.Min(1f,  ( teditobj.Volume2 / 100.0 * ( volumeMod / 100.0)));
      else
        SoundMod.eventsound.Volume =  teditobj.Volume2 / 100f;
    }

    pub static void PlayEventWave(Soundfile: String,  teditobj: EditClass)
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE)
        return;
      str: String = "";
      if (!Information.IsNothing( DrawMod.TGame) && !Information.IsNothing( DrawMod.TGame.Data.SoundDir))
        str = DrawMod.TGame.Data.SoundDir;
      if (Operators.CompareString(str, "", false) == 0)
        str = DrawMod.TGame.ModSoundDir;
      if (!Information.IsNothing( DrawMod.TGame))
        Soundfile = Soundfile.Replace("sound", str);
      if (!teditobj.SoundOn || !File.Exists(Soundfile))
        return;
      SoundMod.eventsound = SoundMod.engine.Play2D(Soundfile, false);
      if (Information.IsNothing( SoundMod.eventsound))
        return;
      SoundMod.eventsound.Volume =  teditobj.Volume2 / 100f;
    }

    pub static void STopEventWave()
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE || Information.IsNothing( SoundMod.eventsound))
        return;
      SoundMod.eventsound.Stop();
    }

    pub static void RestartLastBackground( teditobj: EditClass)
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE || Information.IsNothing( SoundMod.LastSong))
        return;
      lastSong: String = SoundMod.LastSong;
      SoundMod.LastSong = "";
      SoundMod.PlayEventBackground(lastSong,  teditobj);
    }

    pub static void PlayEventBackground(Soundfile: String,  teditobj: EditClass)
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE)
        return;
      Soundfile = Soundfile.Replace("\\", "/");
      Left: String = "";
      if (!Information.IsNothing( DrawMod.TGame) && !Information.IsNothing( DrawMod.TGame.Data.SoundDir))
        Left = DrawMod.TGame.Data.SoundDir;
      if (Operators.CompareString(Left, "", false) == 0)
        Left = DrawMod.TGame.ModSoundDir;
      if (!Information.IsNothing( DrawMod.TGame))
        Soundfile = Soundfile.Replace("/sound/", "/" + Left + "/");
      if (!Information.IsNothing( SoundMod.eventsoundbg) && Operators.CompareString(SoundMod.LastSong, Soundfile, false) == 0 & !SoundMod.eventsoundbg.Finished)
        return;
      SoundMod.LastSong = Soundfile;
      str: String = Soundfile;
      try
      {
        if (!Information.IsNothing( teditobj) & !Information.IsNothing( DrawMod.TGame))
        {
          if (teditobj.AlternateMusic)
          {
            if ( DrawMod.TGame.Data.RuleVar[990] == 1.0)
            {
              if (Strings.InStr(Soundfile, ".") > 0)
                Soundfile = Soundfile.Replace(".", "_alternate.");
            }
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        Soundfile = str;
        ProjectData.ClearProjectError();
      }
      if (!teditobj.IntroSoundOn || !File.Exists(Soundfile))
        return;
      if (!Information.IsNothing( SoundMod.eventsoundbg))
      {
        SoundMod.eventsoundbg.Stop();
        SoundMod.eventsoundbg = (ISound) null;
      }
      SoundMod.eventsoundbg = SoundMod.engine.Play2D(Soundfile, true);
      if (Information.IsNothing( SoundMod.eventsoundbg))
        return;
      SoundMod.eventsoundbg.Volume =  teditobj.Volume / 100f;
    }

    pub static void ChangeEventSoundBg(teditobj: EditClass)
    {
      if (SoundMod.SYSTEMFAILURE)
        return;
      if (!Information.IsNothing( SoundMod.eventsoundbg))
        SoundMod.eventsoundbg.Volume =  teditobj.Volume / 100f;
      let mut index: i32 = 0;
      do
      {
        if (!Information.IsNothing( SoundMod.dssSoundRef[index]))
          SoundMod.dssSoundRef[index].Volume =  teditobj.Volume / 100f;
        index += 1;
      }
      while (index <= 9);
    }

    pub static void ChangeEventSound(teditobj: EditClass)
    {
      if (SoundMod.SYSTEMFAILURE || Information.IsNothing( SoundMod.eventsound))
        return;
      SoundMod.eventsound.Volume =  teditobj.Volume2 / 100f;
    }

    pub static void STopEventBackground()
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE || Information.IsNothing( SoundMod.eventsoundbg))
        return;
      SoundMod.eventsoundbg.Stop();
      SoundMod.eventsoundbg.Dispose();
      SoundMod.eventsoundbg = (ISound) null;
    }

    pub static void StopWave()
    {
      if (SoundMod.NOSOUND)
        return;
      if (SoundMod.SYSTEMFAILURE)
        return;
      try
      {
        SoundMod.engine.StopAllSounds();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
    }

    pub static void EndSound()
    {
      if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE || Information.IsNothing( SoundMod.engine))
        return;
      SoundMod.engine.StopAllSounds();
      SoundMod.engine.RemoveAllSoundSources();
    }

    pub static void dssTimer( teditobj: EditClass)
    {
      if (teditobj.IntroSoundOn & SoundMod.NOSOUND & !SoundMod.SYSTEMFAILURE)
        SoundMod.NOSOUND = false;
      if ( DrawMod.TGame.Data.RuleVar[446] < 1.0 | SoundMod.NOSOUND)
      {
        while (SoundMod.dssSoundCounter > -1)
          SoundMod.dssRemoveSoundSlot(SoundMod.dssSoundCounter);
      }
      else
      {
        if (SoundMod.NOSOUND || SoundMod.SYSTEMFAILURE)
          return;
        if (!Information.IsNothing( SoundMod.eventsoundbg))
        {
          SoundMod.eventsoundbg.Stop();
          SoundMod.eventsoundbg.Dispose();
          SoundMod.eventsoundbg = (ISound) null;
        }
        for (let mut dssSoundCounter: i32 = SoundMod.dssSoundCounter; dssSoundCounter >= 0; dssSoundCounter += -1)
        {
          if (!Information.IsNothing( SoundMod.dssSoundRef[dssSoundCounter]))
          {
            if (SoundMod.dssSoundRef[dssSoundCounter].Finished)
              SoundMod.dssRemoveSoundSlot(dssSoundCounter);
          }
          else
            SoundMod.dssRemoveSoundSlot(dssSoundCounter);
        }
        bool flag = true;
        let mut index1: i32 = -1;
        let mut dssSoundCounter1: i32 = SoundMod.dssSoundCounter;
        for (let mut index2: i32 = 0; index2 <= dssSoundCounter1; index2 += 1)
        {
          if (!Information.IsNothing( SoundMod.dssSoundRef[index2]) && !SoundMod.dssSoundRef[index2].Finished && SoundMod.dssType[index2] == 1)
          {
            flag = false;
            index1 = index2;
          }
        }
        if (flag)
          SoundMod.dssNew( teditobj);
        if (!flag)
          SoundMod.dssAdjust( teditobj);
        if (flag)
          return;
        let mut id: i32 =  Math.Round( DrawMod.TGame.Data.RuleVar[447]);
        let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(id);
        if (stringListById <= -1)
          return;
        let mut length: i32 = DrawMod.TGame.Data.StringListObj[stringListById].Length;
        for (let mut index3: i32 = 0; index3 <= length; index3 += 1)
        {
          if (Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById].Data[index3, 0]) ==  SoundMod.dssId[index1] &&  teditobj.dssLastDominant != Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById].Data[index3, 2]))
          {
            SoundMod.dssEnd( teditobj);
            break;
          }
        }
      }
    }

    pub static void dssEnd( teditobj: EditClass)
    {
      let mut num: i32 =  Math.Round((DateAndTime.Now - SoundMod.startDatetime).TotalMilliseconds);
      for (let mut dssSoundCounter: i32 = SoundMod.dssSoundCounter; dssSoundCounter >= 0; dssSoundCounter += -1)
      {
        if (!Information.IsNothing( SoundMod.dssSoundRef[dssSoundCounter]) & SoundMod.dssStatus[dssSoundCounter] != 3)
        {
          SoundMod.dssStatus[dssSoundCounter] = 3;
          SoundMod.dssMsMark[dssSoundCounter] = num;
        }
      }
    }

    pub static void dssAdjust( teditobj: EditClass)
    {
      bool flag1 = true;
      bool flag2 = true;
      let mut id1: i32 =  Math.Round( DrawMod.TGame.Data.RuleVar[448]);
      let mut stringListById1: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(id1);
      if (stringListById1 < 0 | id1 < 1)
        flag2 = false;
      let mut id2: i32 =  Math.Round( DrawMod.TGame.Data.RuleVar[449]);
      let mut stringListById2: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(id2);
      if (stringListById2 < 0 | id2 < 1)
        flag1 = false;
      if (flag2 & flag1)
      {
        let mut dssSoundCounter1: i32 = SoundMod.dssSoundCounter;
        for (let mut index1: i32 = 0; index1 <= dssSoundCounter1; index1 += 1)
        {
          if (!Information.IsNothing( SoundMod.dssSoundRef[index1]) && !SoundMod.dssSoundRef[index1].Finished && SoundMod.dssType[index1] == 1)
          {
            let mut playPosition: i32 =  SoundMod.dssSoundRef[index1].PlayPosition;
            bool flag3 = true;
            let mut dssSoundCounter2: i32 = SoundMod.dssSoundCounter;
            for (let mut index2: i32 = 0; index2 <= dssSoundCounter2; index2 += 1)
            {
              if (!Information.IsNothing( SoundMod.dssSoundRef[index2]) && !SoundMod.dssSoundRef[index2].Finished && SoundMod.dssType[index2] == 2)
                flag3 = false;
            }
            if (flag3)
            {
              SimpleList simpleList1 = SimpleList::new();
              let mut length1: i32 = DrawMod.TGame.Data.StringListObj[stringListById1].Length;
              for (let mut tid: i32 = 0; tid <= length1; tid += 1)
              {
                if (Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById1].Data[tid, 1]) ==  SoundMod.dssId[index1] && Math.Abs(playPosition - Conversions.ToInteger(DrawMod.TGame.Data.StringListObj[stringListById1].Data[tid, 2])) <= SoundMod.DSS_MARGIN_MS)
                  simpleList1.Add(tid, DrawMod.RandyNumber.Next(0, 100));
              }
              if (simpleList1.Counter > -1)
              {
                simpleList1.Sort();
                let mut index3: i32 = simpleList1.Id[0];
                let mut integer1: i32 = Conversions.ToInteger(DrawMod.TGame.Data.StringListObj[stringListById1].Data[index3, 3]);
                let mut integer2: i32 = Conversions.ToInteger(DrawMod.TGame.Data.StringListObj[stringListById1].Data[index3, 4]);
                SimpleList simpleList2 = SimpleList::new();
                let mut length2: i32 = DrawMod.TGame.Data.StringListObj[stringListById2].Length;
                for (let mut tid: i32 = 0; tid <= length2; tid += 1)
                {
                  if (Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById2].Data[tid, 2]) ==  integer1 && Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById2].Data[tid, 3]) ==  integer2)
                    simpleList2.Add(tid, DrawMod.RandyNumber.Next(0, 100));
                }
                if (simpleList2.Counter > -1)
                {
                  simpleList2.Sort();
                  let mut index4: i32 = simpleList2.Id[0];
                  SoundMod.dssAddSoundSlot();
                  str: String = DrawMod.TGame.Data.StringListObj[stringListById2].Data[index4, 1];
                  SoundMod.dssStartSound( teditobj, SoundMod.dssSoundCounter, DrawMod.TGame.AppPath + "sound/" + str, 2, 2, 1f, Conversions.ToInteger(DrawMod.TGame.Data.StringListObj[stringListById2].Data[index4, 0]), false);
                }
              }
            }
          }
        }
      }
      let mut num1: i32 =  Math.Round((DateAndTime.Now - SoundMod.startDatetime).TotalMilliseconds);
      for (let mut dssSoundCounter: i32 = SoundMod.dssSoundCounter; dssSoundCounter >= 0; dssSoundCounter += -1)
      {
        if (!Information.IsNothing( SoundMod.dssSoundRef[dssSoundCounter]) && !SoundMod.dssSoundRef[dssSoundCounter].Finished && SoundMod.dssStatus[dssSoundCounter] == 3)
        {
          let mut num2: i32 = num1 - SoundMod.dssMsMark[dssSoundCounter];
          if (num2 > SoundMod.DSS_FADE_MS)
            SoundMod.dssRemoveSoundSlot(dssSoundCounter);
          else if (num2 > 0)
            SoundMod.dssSoundRef[dssSoundCounter].Volume =  (1.0 -  num2 /  SoundMod.DSS_FADE_MS) * ( teditobj.Volume / 100f);
        }
      }
    }

    pub static void dssNew( teditobj: EditClass)
    {
      let mut id: i32 =  Math.Round( DrawMod.TGame.Data.RuleVar[447]);
      let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(id);
      if (stringListById < 0 || teditobj.dssLastDominant < 0 & DrawMod.TGame.Data.Product == 7)
        return;
      SimpleList simpleList = SimpleList::new();
      if (!teditobj.IntroSoundOn)
        return;
      let mut length: i32 = DrawMod.TGame.Data.StringListObj[stringListById].Length;
      for (let mut tid: i32 = 0; tid <= length; tid += 1)
      {
        bool flag = false;
        if (Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById].Data[tid, 2]) == -1.0)
          flag = true;
        if (Conversions.ToDouble(DrawMod.TGame.Data.StringListObj[stringListById].Data[tid, 2]) ==  teditobj.dssLastDominant)
          flag = true;
        if (teditobj.dssLastDominant == -1)
          flag = true;
        if (flag)
          simpleList.Add(tid, DrawMod.RandyNumber.Next(0, 100));
      }
      index: i32;
      if (simpleList.Counter > -1)
      {
        simpleList.Sort();
        index = simpleList.Id[0];
      }
      else
        index = -1;
      if (index <= -1)
        return;
      SoundMod.dssAddSoundSlot();
      str: String = DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 1];
      if (simpleList.Counter == 0)
        SoundMod.dssStartSound( teditobj, SoundMod.dssSoundCounter, DrawMod.TGame.AppPath + "sound/" + str, 2, 1, 1f, Conversions.ToInteger(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 0]), true);
      else
        SoundMod.dssStartSound( teditobj, SoundMod.dssSoundCounter, DrawMod.TGame.AppPath + "sound/" + str, 2, 1, 1f, Conversions.ToInteger(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 0]), false);
    }

    pub static void dssStartSound(
       teditobj: EditClass,
      dssSlotNr: i32,
      Soundfile: String,
      tStatus: i32,
      tType: i32,
      float tVolume,
      tId: i32,
      bool tLoop)
    {
      Soundfile = Soundfile.Replace("\\", "/");
      Left: String = "";
      if (!Information.IsNothing( DrawMod.TGame) && !Information.IsNothing( DrawMod.TGame.Data.SoundDir))
        Left = DrawMod.TGame.Data.SoundDir;
      if (Operators.CompareString(Left, "", false) == 0)
        Left = DrawMod.TGame.ModSoundDir;
      if (!Information.IsNothing( DrawMod.TGame))
        Soundfile = Soundfile.Replace("/sound/", "/" + Left + "/");
      if (!teditobj.IntroSoundOn || !File.Exists(Soundfile))
        return;
      if (!Information.IsNothing( SoundMod.dssSoundRef[dssSlotNr]))
      {
        SoundMod.dssSoundRef[dssSlotNr].Stop();
        SoundMod.dssSoundRef[dssSlotNr] = (ISound) null;
      }
      SoundMod.dssSoundRef[dssSlotNr] = !tLoop ? SoundMod.engine.Play2D(Soundfile, false) : SoundMod.engine.Play2D(Soundfile, true);
      if (Information.IsNothing( SoundMod.dssSoundRef[dssSlotNr]))
        return;
      let mut num: i32 =  Math.Round((DateAndTime.Now - SoundMod.startDatetime).TotalMilliseconds);
      SoundMod.dssType[dssSlotNr] = tType;
      SoundMod.dssId[dssSlotNr] = tId;
      SoundMod.dssStatus[dssSlotNr] = tStatus;
      SoundMod.dssMsMark[dssSlotNr] = num;
      SoundMod.dssSoundRef[dssSlotNr].Volume =  ( tVolume *  teditobj.Volume / 100.0);
    }

    pub static void dssAddSoundSlot()
    {
      if (SoundMod.dssSoundCounter >= SoundMod.dssSoundMaxCounter)
      {
        let mut num: i32 =  Interaction.MsgBox( "DSS Sound slot overload");
      }
      else
      {
        SoundMod += 1.dssSoundCounter;
        SoundMod.dssSoundRef[SoundMod.dssSoundCounter] = (ISound) null;
        SoundMod.dssStatus[SoundMod.dssSoundCounter] = 0;
        SoundMod.dssType[SoundMod.dssSoundCounter] = 0;
        SoundMod.dssId[SoundMod.dssSoundCounter] = 0;
        SoundMod.dssMsMark[SoundMod.dssSoundCounter] = 0;
      }
    }

    pub static void dssRemoveSoundSlot(slot: i32)
    {
      if (slot < 0)
      {
        let mut num1: i32 =  Interaction.MsgBox( "Negative DSS Sound slot removal");
      }
      else if (slot > SoundMod.dssSoundCounter)
      {
        let mut num2: i32 =  Interaction.MsgBox( "To high DSS Sound slot removal");
      }
      else
      {
        if (!Information.IsNothing( SoundMod.dssSoundRef[slot]))
        {
          SoundMod.dssSoundRef[slot].Stop();
          SoundMod.dssSoundRef[slot] = (ISound) null;
        }
        let mut num3: i32 = slot;
        let mut dssSoundCounter: i32 = SoundMod.dssSoundCounter;
        for (let mut index: i32 = num3; index <= dssSoundCounter; index += 1)
        {
          if (index + 1 <= SoundMod.dssSoundCounter)
          {
            SoundMod.dssSoundRef[index] = SoundMod.dssSoundRef[index + 1];
            SoundMod.dssStatus[index] = SoundMod.dssStatus[index + 1];
            SoundMod.dssType[index] = SoundMod.dssType[index + 1];
            SoundMod.dssId[index] = SoundMod.dssId[index + 1];
            SoundMod.dssMsMark[index] = SoundMod.dssMsMark[index + 1];
          }
        }
        --SoundMod.dssSoundCounter;
      }
    }
  }
}
