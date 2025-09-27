from dogtail.tree import root
print(list(map(lambda x: x.name, root.applications())))