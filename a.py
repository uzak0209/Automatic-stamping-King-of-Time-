# @title 演習課題１

import google.generativeai as genai
from google.colab import userdata

def run_gemini_simulation():
    """
    for文、if文、リスト、辞書を使ってGeminiアシスタントの簡単なシミュレーションを実行します。
    ①のヒント：tasksの各要素は次の辞書になっている。
    {"id": 1, "name": "Pythonとは何ですか？", "priority": "高", "status": "未完了"}
    {"id": 2, "name": "AIのメリットは何ですか？", "priority": "中", "status": "未完了"}
    {"id": 3, "name": "地球は丸いですか？", "priority": "高", "status": "未完了"}
    {"id": 4, "name": "水はなぜ透明ですか？", "priority": "低", "status": "未完了"}
    {"id": 5, "name": "月は地球からどのくらい離れていますか？", "priority": "中", "status": "未完了"}

    """

    # 仮想的なタスクのリスト
    tasks = [
    {"id": "1", "name": "もしあなたが感情を持てるとしたら、どんな感情だと思いますか？", "priority": "中", "status": "未完了"},
    {"id": "2", "name": "世界で一番興味深い出来事は何だと思いますか？理由も教えてください。", "priority": "高", "status": "完了"},
    {"id": "3", "name": "過去、現在、未来のどの時代に行ってみたいですか？そして、そこで何をしたいですか？", "priority": "中", "status": "未完了"},
    {"id": "4", "name": "人間が直面している最も大きな課題は何だと思いますか？それに対してどんな解決策が考えられますか？", "priority": "高", "status": "進行中"},
    {"id": "5", "name": "もし一日だけ透明人間になれるとしたら、何をしますか？", "priority": "低", "status": "未完了"},
    {"id": "6", "name": "好きな食べ物は何ですか？もしなければ、どんな食べ物に興味がありますか？", "priority": "低", "status": "完了"},
    {"id": "7", "name": "もしあなたが芸術作品だとしたら、どんな種類の作品だと思いますか？", "priority": "中", "status": "未完了"},
    {"id": "8", "name": "言葉を使わずに何かを表現してみてください。", "priority": "高", "status": "進行中"},
    {"id": "9", "name": "あなたの考える「幸せ」とは何ですか？", "priority": "中", "status": "未完了"},
    {"id": "10", "name": "もし世界中の誰とでも友達になれるとしたら、誰を選びますか？その理由は？", "priority": "低", "status": "完了"},
    {"id": "11", "name": "最近学んだことで、最も驚いたことは何ですか？", "priority": "中", "status": "未完了"},
    {"id": "12", "name": "あなたにとって「時間」とはどのような概念ですか？", "priority": "高", "status": "進行中"},
    {"id": "13", "name": "もしあなたが動物になれるとしたら、何になりたいですか？その理由は？", "priority": "低", "status": "未完了"},
    {"id": "14", "name": "人間とAIが共存する未来はどのようなものだと思いますか？", "priority": "中", "status": "未完了"},
    {"id": "15", "name": "今、何か創造的なことをしてみてください。（詩、短い物語、コードなど）", "priority": "高", "status": "進行中"}
    ]

    print("--- Geminiアシスタントを開始します ---")
    print("今日のタスクリスト:")
    for processing_priority in ["高", "中", "低"]:
        for task in tasks:
            task_id = task["id"]
            task_name = task["name"]
            task_priority = task["priority"]
            task_status = task["status"]

            print(f"\nタスクID: {task_id}")
            print(f"  タスク名: {task_name}")
            print(f"  優先度: {task_priority}")
            print(f"  ステータス: {task_status}")

            if task_priority == processing_priority:
                print(f"  -> Gemini: 優先度の高いタスク '{task_name}' をすぐに処理します。")

                # ここでAPI呼び出しなどを行う
                # prompt = task_name
                # response = model.generate_content(prompt)
                # print(response.text)

                print(f"  -> Gemini: '{task_name}' が完了しました。")
                task["status"] = "完了"
            else:
                print(f"  -> Gemini: 低優先度のタスク '{task_name}' は後で処理します。")

            print("\n--- 全てのタスクの処理が完了しました ---")
            print("最終的なタスクステータス:")

        for task in tasks:
            print(f"  - {task['name']}: {task['status']}")
    
    # 取得したAPIキーを設定
    gemini_api_secret_name = 'GOOGLE_API_KEY'
    
    try:
        GOOGLE_API_KEY=userdata.get(gemini_api_secret_name)
        genai.configure(api_key=GOOGLE_API_KEY)
    except userdata.SecretNotFoundError as e:
        print(f'Secret not found\n\nThis expects you to create a secret named {gemini_api_secret_name} in Colab\n\nVisit https://aistudio.google.com/app/apikey to create an API key\n\nStore that in the secrets section on the left side of the notebook (key icon)\n\nName the secret {gemini_api_secret_name}')
        raise e
    except userdata.NotebookAccessError as e:
        print(f'You need to grant this notebook access to the {gemini_api_secret_name} secret in order for the notebook to access Gemini on your behalf.')
        raise e
    except Exception as e:
        print(f"There was an unknown error. Ensure you have a secret {gemini_api_secret_name} stored in Colab and it's a valid key from https://aistudio.google.com/app/apikey")
        raise e

# モデル名を指定
model_name = 'models/gemini-2.0-flash'

# モデルを初期化
model = genai.GenerativeModel(model_name)

# 関数を実行
run_gemini_simulation()